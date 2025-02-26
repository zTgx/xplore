use crate::error::Result;
use async_trait::async_trait;
use auth::UserAuth;
use primitives::{Profile, Tweet};
use reqwest::Client;

pub mod auth;
pub mod error;
pub mod http;
pub mod primitives;
pub mod profile;
pub mod relationships;
pub mod search;
pub mod timeline;
pub mod tweets;

#[async_trait]
pub trait IProfile {
    async fn get_profile_by_screen_name(&self, screen_name: &str) -> Result<Profile>;
    async fn get_screen_name_by_user_id(&self, user_id: &str) -> Result<String>;
    async fn get_user_id_by_screen_name(&self, screen_name: &str) -> Result<String>;
}

#[derive(Clone)]
pub struct Xplore {
    pub client: Client,
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
    pub async fn send_tweet(&self, text: &str, media_ids: Option<Vec<String>>) -> Result<Tweet> {
        let mut params = serde_json::json!({
            "text": text,
        });

        if let Some(ids) = media_ids {
            params["media"] = serde_json::json!({ "media_ids": ids });
        }

        let endpoint = "https://api.twitter.com/2/tweets";
        self.post(endpoint, Some(params)).await
    }

    pub async fn get_tweet(&self, tweet_id: &str) -> Result<Tweet> {
        let endpoint = format!("https://api.twitter.com/2/tweets/{}", tweet_id);
        self.get(&endpoint).await
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
