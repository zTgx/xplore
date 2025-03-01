use async_trait::async_trait;
use cookie::CookieTracker;
use inner::Inner;
use primitives::{Profile, Tweet, TweetRetweetResponse};
use search::SearchMode;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
use timeline::v1::{QueryProfilesResponse, QueryTweetsResponse};

pub mod auth;
pub mod cookie;
pub mod inner;
pub mod primitives;
pub mod profile;
pub mod relationships;
pub mod rpc;
pub mod search;
pub mod timeline;
pub mod tweets;

// #[derive(Clone)]
pub struct Xplore {
    inner: Inner,
    pub cookie_tracker: CookieTracker,
}

impl Xplore {
    pub async fn new(cookie: &str) -> Result<Self> {
        let inner = Inner::new(cookie).await?;
        let cookie_tracker = CookieTracker::new(cookie);

        Ok(Self { inner, cookie_tracker })
    }
}

#[async_trait]
pub trait IProfile {
    async fn get_profile(&self, screen_name: &str) -> Result<Profile>;
    async fn get_user_id(&self, screen_name: &str) -> Result<String>;
}

#[async_trait]
pub trait ITweet {
    async fn post_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value>;

    async fn read_tweet(&self, tweet_id: &str) -> Result<Tweet>;
    async fn retweet(&self, tweet_id: &str) -> Result<TweetRetweetResponse>;
    async fn like_tweet(&self, tweet_id: &str) -> Result<Value>;

    async fn get_user_tweets(&self, user_id: &str, limit: usize) -> Result<Vec<Tweet>>;
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

#[async_trait]
pub trait IRel {
    async fn following(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)>;

    async fn followers(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)>;

    async fn follow(&self, username: &str) -> Result<()>;
    async fn unfollow(&self, username: &str) -> Result<()>;
}

#[derive(Debug, Error, Deserialize)]
pub enum TwitterError {
    #[error("API error: {0}")]
    Api(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Network error: {0}")]
    #[serde(skip)]
    Network(#[from] reqwest::Error),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Missing environment variable: {0}")]
    EnvVar(String),

    #[error("Cookie error: {0}")]
    Cookie(String),

    #[error("JSON error: {0}")]
    #[serde(skip)]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    #[serde(skip)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TwitterError>;
