use crate::error::Result;
use async_trait::async_trait;
use auth::UserAuth;
use cookie::CookieTracker;
use inner::Inner;
use primitives::{Profile, Tweet, TweetRetweetResponse};
use reqwest::Client;
use search::SearchMode;
use serde_json::Value;
use timeline::v1::{QueryProfilesResponse, QueryTweetsResponse};

pub mod auth;
pub mod error;
pub mod http;
pub mod primitives;
pub mod profile;
pub mod relationships;
pub mod search;
pub mod timeline;
pub mod tweets;

pub mod cookie;
pub mod inner;
pub mod rpc;

// #[derive(Clone)]
pub struct XYZ {
    inner: Inner,
    pub cookie_tracker: CookieTracker,
}

impl XYZ {
    pub async fn new(cookie: &str) -> Result<Self> {
        let inner = Inner::new(cookie).await?;
        let cookie_tracker = CookieTracker::new(cookie);

        Ok(Self { inner, cookie_tracker })
    }
}

#[async_trait]
pub trait IXYZProfile {
    async fn get_profile(&self, screen_name: &str) -> Result<Profile>;
    async fn get_user_id(&self, screen_name: &str) -> Result<String>;
}

#[async_trait]
pub trait IXYZTweet {
    async fn post_tweet(&self, text: &str);
}

#[async_trait]
pub trait ISearch {
    async fn search_tweets(
        &self,
        query: &str,
        max_tweets: i32,
        search_mode: SearchMode,
        cursor: Option<String>,
    ) -> Result<QueryTweetsResponse>;

    async fn search_profiles(
        &self,
        query: &str,
        max_profiles: i32,
        cursor: Option<String>,
    ) -> Result<QueryProfilesResponse>;
}

/// `Xplore` struct represents the core components needed for the application.
/// It contains the client for making requests and the authentication details.
#[derive(Clone)]
pub struct Xplore {
    /// The client used to interact with external services.
    pub client: Client,

    /// The authentication details for user access.
    pub auth: UserAuth,
}

impl Xplore {
    pub async fn new(cookie: &str) -> Result<Self> {
        let client = xplore_utils::client()?;
        let auth = xplore_utils::make_auth(cookie).await?;

        Ok(Xplore { client, auth })
    }
}

impl Xplore {
    pub async fn post_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        crate::tweets::create_tweet_request(&self, text, reply_to, media_data).await
    }

    pub async fn read_tweet(&self, tweet_id: &str) -> Result<Tweet> {
        crate::tweets::get_tweet(&self, tweet_id).await
    }

    pub async fn retweet(&self, tweet_id: &str) -> Result<TweetRetweetResponse> {
        let value = crate::tweets::retweet(&self, tweet_id).await?;
        let res = serde_json::from_value(value)?;

        Ok(res)
    }

    pub async fn get_user_tweets(&self, user_id: &str, limit: usize) -> Result<Vec<Tweet>> {
        let endpoint = format!("https://api.twitter.com/2/users/{}/tweets", user_id);
        let params = serde_json::json!({
            "max_results": limit,
            "tweet.fields": "created_at,author_id,conversation_id,public_metrics"
        });
        self.get_with_params(&endpoint, Some(params)).await
    }
}

mod xplore_utils {
    use crate::{
        auth::UserAuth,
        error::{Result, TwitterError},
    };
    use reqwest::Client;
    use std::time::Duration;

    pub fn client() -> Result<Client> {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .map_err(|e| TwitterError::Network(e))
    }

    pub async fn make_auth(cookie: &str) -> Result<UserAuth> {
        let mut auth = UserAuth::new().await?;

        auth.set_from_cookie_string(cookie).await?;

        Ok(auth)
    }
}
