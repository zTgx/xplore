use {
    crate::{
        cookie::CookieTracker,
        inner::Inner,
        primitives::{Profile, Result, Tweet, TweetRetweetResponse},
        search::SearchMode,
        timeline::v1::{QueryProfilesResponse, QueryTweetsResponse},
        timeline::v2::{QueryTweetsResponse as V2QueryTweetsResponse},
    },
    async_trait::async_trait,
    serde_json::Value,
};

pub struct Xplore {
    pub inner: Inner,
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

    async fn send_quote_tweet(
        &self,
        text: &str,
        quoted_tweet_id: &str,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value>;

    async fn fetch_tweets_and_replies(
        &self,
        username: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<V2QueryTweetsResponse>;

    async fn fetch_tweets_and_replies_by_user_id(
        &self,
        user_id: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<V2QueryTweetsResponse>;

    async fn fetch_list_tweets(
        &self,
        list_id: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<Value>;

    async fn create_long_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_ids: Option<Vec<String>>,
    ) -> Result<Value>;
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

#[async_trait]
pub trait IHome {
    async fn get_home_timeline(
        &self,
        count: i32,
        seen_tweet_ids: Vec<String>,
    ) -> Result<Vec<Value>>;
}
