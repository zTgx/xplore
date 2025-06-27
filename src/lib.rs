//! Xplore - A Twitter API client for Rust
//!
//! This crate provides a convenient way to interact with Twitter's API.

pub mod core;
pub mod services;
pub mod utils;

///! Export
pub use core::models::search::SearchMode;

use {
    crate::{
        core::{
            inner::Inner,
            models::{
                profile::Profile,
                timeline_v1::{QueryProfilesResponse, QueryTweetsResponse},
                timeline_v2::QueryTweetsResponse as V2QueryTweetsResponse,
                tweets::{Tweet, TweetRetweetResponse},
                Result,
            },
        },
        services::{home, profile, relationship, search, tweet},
    },
    serde_json::Value,
};

///! TODO: refactor Xplore fields???
pub struct Xplore {
    pub inner: Inner,
}

impl Xplore {
    pub async fn new(cookie: &str) -> Result<Self> {
        let inner = Inner::new(cookie).await?;

        Ok(Self { inner })
    }
}

impl Xplore {
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
        email: Option<&str>,
        two_factor_secret: Option<&str>,
    ) -> Result<bool> {
        todo!()
    }

    pub async fn logout() -> Result<bool> {
        todo!()
    }

    pub async fn set_cookies(_cookies: Vec<String>) {
        todo!()
    }

    pub async fn get_coookies() -> Result<Vec<String>> {
        todo!()
    }
}

///! Profile's API collection
impl Xplore {
    ///! Fetches the profile of a user by their screen name.
    /// # Arguments
    /// * `screen_name` - The screen name of the user whose profile is to be fetched.
    /// # Returns
    /// * `Result<Profile>` - A result containing the user's profile if successful, or an error if not.
    /// # Errors
    /// Returns an error if the profile cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn get_profile(&self, screen_name: &str) -> Result<Profile> {
        profile::get_profile(&self, screen_name).await
    }

    ///! Fetches the user ID of a user by their screen name.
    /// # Arguments
    /// * `screen_name` - The screen name of the user whose ID is to be fetched.
    /// # Returns
    /// * `Result<String>` - A result containing the user's ID if successful, or an error if not.
    /// # Errors
    /// Returns an error if the user ID cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn get_user_id(&self, screen_name: &str) -> Result<String> {
        profile::get_user_id(&self, screen_name).await
    }
}

///! Search's API collection
impl Xplore {
    ///! Searches for tweets based on a query string.
    /// # Arguments
    /// * `query` - The search query string to find tweets.
    /// * `max_tweets` - The maximum number of tweets to return.
    /// * `search_mode` - The mode of search to be used (e.g., recent, popular).
    /// * `cursor` - An optional cursor for pagination.
    /// # Returns
    /// * `Result<QueryTweetsResponse>` - A result containing the response with tweets if successful, or an error if not.
    /// # Errors
    /// Returns an error if the search fails, such as if the query is invalid or if there is a network issue.
    pub async fn search_tweets(
        &self,
        query: &str,
        max_tweets: i32,
        search_mode: SearchMode,
        cursor: Option<String>,
    ) -> Result<QueryTweetsResponse> {
        search::search_tweets(&self, query, max_tweets, search_mode, cursor).await
    }

    ///! Searches for user profiles based on a query string.
    /// # Arguments
    /// * `query` - The search query string to find user profiles.
    /// * `max_profiles` - The maximum number of profiles to return.
    /// * `cursor` - An optional cursor for pagination.
    /// # Returns
    /// * `Result<QueryProfilesResponse>` - A result containing the response with user profiles if successful, or an error if not.
    /// # Errors
    /// Returns an error if the search fails, such as if the query is invalid or if there is a network issue.
    pub async fn search_profiles(
        &self,
        query: &str,
        max_profiles: i32,
        cursor: Option<String>,
    ) -> Result<QueryProfilesResponse> {
        search::search_profiles(&self, query, max_profiles, cursor).await
    }
}

///! Relationship's API collection
impl Xplore {
    ///! Fetches the relationship status between the authenticated user and another user.
    /// # Arguments
    /// * `user_id` - The ID of the user whose relationship status is to be fetched.
    /// # Returns
    /// * `Result<Profile>` - A result containing the profile of the user if successful, or an error if not.
    /// # Errors
    /// Returns an error if the relationship status cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn get_following(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        relationship::get_following(&self, user_id, count, cursor).await
    }

    ///! Fetches the followers of a user.
    /// # Arguments
    /// * `user_id` - The ID of the user whose followers are to be fetched
    /// * `count` - The maximum number of followers to return.
    /// * `cursor` - An optional cursor for pagination.
    /// # Returns
    /// * `Result<(Vec<Profile>, Option<String>)>` - A result containing a tuple with a vector of profiles and an optional cursor for pagination if successful, or an error if not
    /// # Errors
    /// Returns an error if the followers cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn get_followers(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        relationship::get_followers(&self, user_id, count, cursor).await
    }

    ///! Follows a user by their username.
    /// # Arguments
    /// * `username` - The username of the user to follow.
    /// # Returns
    /// * `Result<()>` - A result indicating success or failure.
    /// # Errors    
    /// Returns an error if the follow action fails, such as if the user does not exist or if there is a network issue.
    pub async fn follow(&self, username: &str) -> Result<()> {
        relationship::follow(&self, username).await
    }

    ///! Unfollows a user by their username.
    /// # Arguments
    /// * `username` - The username of the user to unfollow.
    /// # Returns
    /// * `Result<()>` - A result indicating success or failure.
    /// # Errors
    /// Returns an error if the unfollow action fails, such as if the user does not
    pub async fn unfollow(&self, username: &str) -> Result<()> {
        relationship::unfollow(&self, username).await
    }
}

///! Tweet's API collection
impl Xplore {
    ///! Posts a tweet with optional media attachments.
    /// # Arguments
    /// * `text` - The text content of the tweet.
    /// * `reply_to` - An optional tweet ID to reply to.
    /// * `media_data` - An optional vector of tuples containing media data and their corresponding media types.
    /// # Returns
    /// * `Result<Value>` - A result containing the response from the tweet posting if successful, or an error if not.
    /// # Errors
    /// Returns an error if the tweet cannot be posted, such as if the text is too long, if the media data is invalid, or if there is a network issue.
    pub async fn post_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        tweet::post_tweet(&self, text, reply_to, media_data).await
    }

    ///! reads a tweet by its ID.
    /// # Arguments
    /// * `tweet_id` - The ID of the tweet to be read.
    /// # Returns
    /// * `Result<Tweet>` - A result containing the tweet if successful, or an error if not.
    /// # Errors
    /// Returns an error if the tweet cannot be read, such as if the tweet does not exist or if there is a network issue.
    pub async fn read_tweet(&self, tweet_id: &str) -> Result<Tweet> {
        tweet::read_tweet(&self, tweet_id).await
    }

    ///! Retweets a tweet by its ID.
    /// # Arguments
    /// * `tweet_id` - The ID of the tweet to be retweeted.
    /// # Returns
    /// * `Result<TweetRetweetResponse>` - A result containing the retweet response if successful, or an error if not.
    /// # Errors
    /// Returns an error if the retweet action fails, such as if the tweet does not exist, if the user has already retweeted it, or if there is a network issue.
    pub async fn retweet(&self, tweet_id: &str) -> Result<TweetRetweetResponse> {
        tweet::retweet(&self, tweet_id).await
    }

    ///! Likes a tweet by its ID.
    /// # Arguments
    /// * `tweet_id` - The ID of the tweet to be liked.
    /// # Returns
    /// * `Result<Value>` - A result containing the response from the like action if successful, or an error if not.
    /// # Errors
    /// Returns an error if the like action fails, such as if the tweet does not exist, if the user has already liked it, or if there is a network issue.
    pub async fn like_tweet(&self, tweet_id: &str) -> Result<Value> {
        tweet::like_tweet(&self, tweet_id).await
    }

    ///! Gets a user's tweets.
    /// # Arguments
    /// * `user_id` - The ID of the user whose tweets are to be fetched.
    /// * `limit` - The maximum number of tweets to return.
    /// # Returns
    /// * `Result<Vec<Tweet>>` - A result containing a vector of tweets if successful, or an error if not.
    /// # Errors
    /// Returns an error if the tweets cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn get_user_tweets(&self, user_id: &str, limit: usize) -> Result<Vec<Tweet>> {
        tweet::get_user_tweets(&self, user_id, limit).await
    }

    ///! Sends a quote tweet with optional media attachments.
    /// # Arguments
    /// * `text` - The text content of the quote tweet.
    /// * `quoted_tweet_id` - The ID of the tweet being quoted.
    /// * `media_data` - An optional vector of tuples containing media data and their corresponding media types.
    /// # Returns
    /// * `Result<Value>` - A result containing the response from the quote tweet action if successful, or an error if not.
    /// # Errors
    /// Returns an error if the quote tweet cannot be sent, such as if the text is too long, if the quoted tweet does not exist, if the media data is invalid, or if there is a network issue.
    pub async fn send_quote_tweet(
        &self,
        text: &str,
        quoted_tweet_id: &str,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        tweet::send_quote_tweet(&self, text, quoted_tweet_id, media_data).await
    }

    ///! Fetches tweets and replies from a user's timeline.
    /// # Arguments
    /// * `username` - The screen name of the user whose tweets and replies are to be fetched.
    /// * `max_tweets` - The maximum number of tweets to return.
    /// * `cursor` - An optional cursor for pagination.
    /// # Returns
    /// * `Result<V2QueryTweetsResponse>` - A result containing the response with tweets and replies if successful, or an error if not.
    /// # Errors
    /// Returns an error if the tweets and replies cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn fetch_tweets_and_replies(
        &self,
        username: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<V2QueryTweetsResponse> {
        tweet::fetch_tweets_and_replies(&self, username, max_tweets, cursor).await
    }

    ///! Fetches tweets and replies from a user's timeline by their user ID.
    /// # Arguments
    /// * `user_id` - The ID of the user whose tweets and replies are to be fetched.
    /// * `max_tweets` - The maximum number of tweets to return.
    /// * `cursor` - An optional cursor for pagination.
    /// # Returns
    /// * `Result<V2QueryTweetsResponse>` - A result containing the response with tweets and replies if successful, or an error if not.
    /// # Errors
    /// Returns an error if the tweets and replies cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn fetch_tweets_and_replies_by_user_id(
        &self,
        user_id: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<V2QueryTweetsResponse> {
        tweet::fetch_tweets_and_replies_by_user_id(&self, user_id, max_tweets, cursor).await
    }

    ///! Fetches tweets from a list by its ID.
    /// # Arguments
    /// * `list_id` - The ID of the list whose tweets are to be fetched.
    /// * `max_tweets` - The maximum number of tweets to return.
    /// * `cursor` - An optional cursor for pagination.
    /// # Returns
    /// * `Result<Value>` - A result containing the response with tweets if successful, or an error if not.
    /// # Errors
    /// Returns an error if the tweets cannot be fetched, such as if the list does not exist or if there is a network issue.
    pub async fn fetch_list_tweets(&self, list_id: &str, max_tweets: i32, cursor: Option<&str>) -> Result<Value> {
        tweet::fetch_list_tweets(&self, list_id, max_tweets, cursor).await
    }

    ///! Creates a long tweet with optional media attachments.
    /// # Arguments
    /// * `text` - The text content of the long tweet.
    /// * `reply_to` - An optional tweet ID to reply to.
    /// * `media_ids` - An optional vector of media IDs to attach to the long tweet.
    /// # Returns
    /// * `Result<Value>` - A result containing the response from the long tweet creation if successful, or an error if not.
    /// # Errors
    /// Returns an error if the long tweet cannot be created, such as if the text is too long, if the media IDs are invalid, or if there is a network issue.
    pub async fn create_long_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_ids: Option<Vec<String>>,
    ) -> Result<Value> {
        tweet::create_long_tweet(&self, text, reply_to, media_ids).await
    }
}

///! Home's API collection
impl Xplore {
    ///! Fetches the home timeline with a specified count and a list of seen tweet IDs.
    /// # Arguments
    /// * `count` - The number of tweets to return.
    /// * `seen_tweet_ids` - A vector of tweet IDs that have already been seen.
    /// # Returns
    /// * `Result<Vec<Value>>` - A result containing a vector of tweets if successful, or an error if not.
    /// # Errors
    /// Returns an error if the home timeline cannot be fetched, such as if there is a network issue or if the user is not authenticated.
    pub async fn get_home_timeline(&self, count: i32, seen_tweet_ids: Vec<String>) -> Result<Vec<Value>> {
        home::get_home_timeline(&self, count, seen_tweet_ids).await
    }
}
