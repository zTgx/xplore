use crate::api::client::Xplore;
use crate::auth::user_auth::UserAuth;
use crate::primitives::constants::BEARER_TOKEN;
use crate::error::Result;
use crate::error::TwitterError;
use crate::primitives::{Profile, Tweet};
use crate::search::{fetch_search_tweets, SearchMode};
use crate::timeline::v1::{QueryProfilesResponse, QueryTweetsResponse};
use crate::timeline::v2::QueryTweetsResponse as V2QueryTweetsResponse;
use serde_json::Value;

pub struct Scraper {
    pub twitter_client: Xplore,
}

impl Scraper {
    pub async fn get_home_timeline(
        &self,
        count: i32,
        seen_tweet_ids: Vec<String>,
    ) -> Result<Vec<Value>> {
        crate::timeline::home::fetch_home_timeline(&self.twitter_client, count, seen_tweet_ids).await
    }

    pub async fn send_quote_tweet(
        &self,
        text: &str,
        quoted_tweet_id: &str,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        crate::tweets::create_quote_tweet(&self.twitter_client, text, quoted_tweet_id, media_data).await
    }

    pub async fn fetch_tweets_and_replies(
        &self,
        username: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<V2QueryTweetsResponse> {
            crate::tweets::fetch_tweets_and_replies(&self.twitter_client, username, max_tweets, cursor).await
    }
    pub async fn fetch_tweets_and_replies_by_user_id(
        &self,
        user_id: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<V2QueryTweetsResponse> {
        crate::tweets::fetch_tweets_and_replies_by_user_id(&self.twitter_client, user_id, max_tweets, cursor).await
    }
    pub async fn fetch_list_tweets(
        &self,
        list_id: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<Value> {
        crate::tweets::fetch_list_tweets(&self.twitter_client, list_id, max_tweets, cursor).await
    }

    pub async fn like_tweet(&self, tweet_id: &str) -> Result<Value> {
        crate::tweets::like_tweet(&self.twitter_client, tweet_id).await
    }

    pub async fn retweet(&self, tweet_id: &str) -> Result<Value> {
        crate::tweets::retweet(&self.twitter_client, tweet_id).await
    }

    pub async fn create_long_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_ids: Option<Vec<String>>,
    ) -> Result<Value> {
        crate::tweets::create_long_tweet(&self.twitter_client, text, reply_to, media_ids).await
    }

    pub async fn get_tweet(&self, id: &str) -> Result<Tweet> {
        crate::tweets::get_tweet(&self.twitter_client, id).await
    }

    pub async fn search_tweets(
        &self,
        query: &str,
        max_tweets: i32,
        search_mode: SearchMode,
        cursor: Option<String>,
    ) -> Result<QueryTweetsResponse> {
        fetch_search_tweets(&self.twitter_client, query, max_tweets, search_mode, cursor).await
    }

    pub async fn search_profiles(
        &self,
        query: &str,
        max_profiles: i32,
        cursor: Option<String>,
    ) -> Result<QueryProfilesResponse> {
        crate::search::search_profiles(&self.twitter_client, query, max_profiles, cursor).await
    }

    pub async fn get_user_tweets(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<V2QueryTweetsResponse> {
        crate::tweets::fetch_user_tweets(&self.twitter_client, user_id, count, cursor.as_deref()).await
    }
}
