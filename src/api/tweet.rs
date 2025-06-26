use {
    crate::core::models::{
        timeline_v2::QueryTweetsResponse as V2QueryTweetsResponse,
        tweets::{Tweet, TweetRetweetResponse},
        Result,
    },
    async_trait::async_trait,
    serde_json::Value,
};

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

    async fn fetch_list_tweets(&self, list_id: &str, max_tweets: i32, cursor: Option<&str>) -> Result<Value>;

    async fn create_long_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_ids: Option<Vec<String>>,
    ) -> Result<Value>;
}
