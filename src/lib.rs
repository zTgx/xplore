//! Xplore - A X API client for Rust
//!
//! This crate provides a convenient way to interact with X's undocumented API.

mod api;
mod api_utils;
mod auth;
mod endpoints;
pub mod profile;
mod rate_limit;
pub mod relationship;
pub mod search;
mod timeline_v1;
mod timeline_v2;
mod trend;
pub mod tweets;

use {
    crate::{
        auth::UserAuth,
        profile::{get_profile, get_user_id, Profile},
        rate_limit::RateLimitStrategy,
        search::SearchMode,
        timeline_v1::{QueryProfilesResponse, QueryTweetsResponse},
        timeline_v2::QueryTweetsResponse as V2QueryTweetsResponse,
        tweets::{
            create_long_tweet, fetch_list_tweets, fetch_tweets_and_replies, fetch_tweets_and_replies_by_user_id,
            get_user_tweets, like_tweet, post_tweet, read_tweet, retweet, send_quote_tweet, Tweet,
            TweetRetweetResponse,
        },
    },
    chrono::Duration,
    serde::Deserialize,
    serde_json::Value,
    thiserror::Error,
};

pub type Result<T> = std::result::Result<T, XploreError>;

#[derive(Debug, Error, Deserialize)]
pub enum XploreError {
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

/// Configuration options for the Xplore scraper.
pub struct XploreOptions {
    /// The rate limiting strategy to use when the scraper hits API limits.
    pub rate_limit_strategy: Box<dyn RateLimitStrategy>,

    /// Timeout for individual requests.
    ///
    /// Default: 30 seconds
    pub request_timeout: Duration,

    /// Maximum number of retries for failed requests.
    ///
    /// Default: 3
    pub max_retries: u32,

    /// Whether to automatically follow redirects.
    ///
    /// Default: true
    pub follow_redirects: bool,
}

pub struct Xplore {
    auth: UserAuth,
}

impl Xplore {
    pub async fn new(_options: Option<XploreOptions>) -> Result<Self> {
        let auth = UserAuth::new().await?;
        Ok(Self { auth })
    }
}

///! Login's API collection
impl Xplore {
    ///! Login Method
    ///
    /// Authenticates a user with the provided credentials.
    ///
    /// # Arguments
    /// * `username` - The username of the user attempting to log in.
    /// * `password` - The password of the user attempting to log in.
    /// * `email` - Optional email address for additional authentication (if required by the service).
    /// * `two_factor_secret` - Optional two-factor authentication secret (if 2FA is enabled).
    ///
    /// # Returns
    /// * `Result<bool>` - Returns `Ok(true)` if login was successful, or an error if authentication failed.
    ///
    /// # Errors
    /// Returns an error if:
    /// - Invalid credentials were provided
    /// - Two-factor authentication failed
    /// - Network error occurred during authentication
    /// - Server rejected the login request
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
        email: Option<&str>,
        two_factor_secret: Option<&str>,
    ) -> Result<bool> {
        let _ = self.auth.login(username, password, email, two_factor_secret).await;

        Ok(true)
    }

    ///! Logout Method
    ///
    /// Terminates the current user session.
    ///
    /// # Returns
    /// * `Result<bool>` - Returns `Ok(true)` if logout was successful, or an error if logout failed.
    ///
    /// # Errors
    /// Returns an error if:
    /// - No active session was found
    /// - Network error occurred during logout
    /// - Server rejected the logout request
    pub async fn logout(&mut self) -> Result<bool> {
        self.auth.logout().await;

        Ok(true)
    }

    ///! Set Cookie Method
    ///
    /// Sets the authentication cookie from a raw cookie string.
    ///
    /// # Arguments
    /// * `cookie` - The raw cookie string containing authentication information.
    ///
    /// # Returns
    /// This method does not return a value, but may fail silently if the cookie is invalid.
    ///
    /// # Errors
    /// Errors may occur internally during cookie parsing and validation, but are currently ignored.
    pub async fn set_cookie(&mut self, cookie: &str) {
        let _ = self.auth.set_from_cookie_string(cookie).await;
    }

    ///! Get Cookie Method
    ///
    /// Retrieves the current authentication cookie as a string.
    ///
    /// # Returns
    /// * `Result<String>` - Returns `Ok(String)` containing the cookie if available, or an error if the cookie could not be retrieved.
    ///
    /// # Errors
    /// Returns an error if:
    /// - No active session exists
    /// - Cookie could not be serialized to string
    /// - Network error occurred during cookie retrieval
    pub async fn get_cookie(&mut self) -> Result<String> {
        self.auth.get_cookie_string().await
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
    pub async fn get_profile(&mut self, screen_name: &str) -> Result<Profile> {
        get_profile(&mut self.auth, screen_name).await
    }

    ///! Fetches the user ID of a user by their screen name.
    /// # Arguments
    /// * `screen_name` - The screen name of the user whose ID is to be fetched.
    /// # Returns
    /// * `Result<String>` - A result containing the user's ID if successful, or an error if not.
    /// # Errors
    /// Returns an error if the user ID cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn get_user_id(&mut self, screen_name: &str) -> Result<String> {
        get_user_id(&mut self.auth, screen_name).await
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
        &mut self,
        query: &str,
        max_tweets: i32,
        search_mode: SearchMode,
        cursor: Option<String>,
    ) -> Result<QueryTweetsResponse> {
        search::search_tweets(&mut self.auth, query, max_tweets, search_mode, cursor).await
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
        &mut self,
        query: &str,
        max_profiles: i32,
        cursor: Option<String>,
    ) -> Result<QueryProfilesResponse> {
        search::search_profiles(&mut self.auth, query, max_profiles, cursor).await
    }
}

///! Relationship's API collection
impl Xplore {
    ///! Fetches the home timeline with a specified count and a list of seen tweet IDs.
    /// # Arguments
    /// * `count` - The number of tweets to return.
    /// * `seen_tweet_ids` - A vector of tweet IDs that have already been seen.
    /// # Returns
    /// * `Result<Vec<Value>>` - A result containing a vector of tweets if successful, or an error if not.
    /// # Errors
    /// Returns an error if the home timeline cannot be fetched, such as if there is a network issue or if the user is not authenticated.
    pub async fn get_home_timeline(&mut self, count: i32, seen_tweet_ids: Vec<String>) -> Result<Vec<Value>> {
        relationship::get_home_timeline(self, count, seen_tweet_ids).await
    }

    ///! Fetches the relationship status between the authenticated user and another user.
    /// # Arguments
    /// * `user_id` - The ID of the user whose relationship status is to be fetched.
    /// # Returns
    /// * `Result<Profile>` - A result containing the profile of the user if successful, or an error if not.
    /// # Errors
    /// Returns an error if the relationship status cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn get_following(
        &mut self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        relationship::get_following(self, user_id, count, cursor).await
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
        &mut self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        relationship::get_followers(self, user_id, count, cursor).await
    }

    ///! Follows a user by their username.
    /// # Arguments
    /// * `username` - The username of the user to follow.
    /// # Returns
    /// * `Result<()>` - A result indicating success or failure.
    /// # Errors    
    /// Returns an error if the follow action fails, such as if the user does not exist or if there is a network issue.
    pub async fn follow(&mut self, username: &str) -> Result<()> {
        relationship::follow(self, username).await
    }

    ///! Unfollows a user by their username.
    /// # Arguments
    /// * `username` - The username of the user to unfollow.
    /// # Returns
    /// * `Result<()>` - A result indicating success or failure.
    /// # Errors
    /// Returns an error if the unfollow action fails, such as if the user does not
    pub async fn unfollow(&mut self, username: &str) -> Result<()> {
        relationship::unfollow(self, username).await
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
        &mut self,
        text: &str,
        reply_to: Option<&str>,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        post_tweet(self, text, reply_to, media_data).await
    }

    ///! reads a tweet by its ID.
    /// # Arguments
    /// * `tweet_id` - The ID of the tweet to be read.
    /// # Returns
    /// * `Result<Tweet>` - A result containing the tweet if successful, or an error if not.
    /// # Errors
    /// Returns an error if the tweet cannot be read, such as if the tweet does not exist or if there is a network issue.
    pub async fn read_tweet(&mut self, tweet_id: &str) -> Result<Tweet> {
        read_tweet(self, tweet_id).await
    }

    ///! Retweets a tweet by its ID.
    /// # Arguments
    /// * `tweet_id` - The ID of the tweet to be retweeted.
    /// # Returns
    /// * `Result<TweetRetweetResponse>` - A result containing the retweet response if successful, or an error if not.
    /// # Errors
    /// Returns an error if the retweet action fails, such as if the tweet does not exist, if the user has already retweeted it, or if there is a network issue.
    pub async fn retweet(&mut self, tweet_id: &str) -> Result<TweetRetweetResponse> {
        retweet(self, tweet_id).await
    }

    ///! Likes a tweet by its ID.
    /// # Arguments
    /// * `tweet_id` - The ID of the tweet to be liked.
    /// # Returns
    /// * `Result<Value>` - A result containing the response from the like action if successful, or an error if not.
    /// # Errors
    /// Returns an error if the like action fails, such as if the tweet does not exist, if the user has already liked it, or if there is a network issue.
    pub async fn like_tweet(&mut self, tweet_id: &str) -> Result<Value> {
        like_tweet(self, tweet_id).await
    }

    ///! Gets a user's tweets.
    /// # Arguments
    /// * `user_id` - The ID of the user whose tweets are to be fetched.
    /// * `limit` - The maximum number of tweets to return.
    /// # Returns
    /// * `Result<Vec<Tweet>>` - A result containing a vector of tweets if successful, or an error if not.
    /// # Errors
    /// Returns an error if the tweets cannot be fetched, such as if the user does not exist or if there is a network issue.
    pub async fn get_user_tweets(&mut self, user_id: &str, limit: usize) -> Result<Vec<Tweet>> {
        get_user_tweets(self, user_id, limit).await
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
        &mut self,
        text: &str,
        quoted_tweet_id: &str,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        send_quote_tweet(self, text, quoted_tweet_id, media_data).await
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
        &mut self,
        username: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<V2QueryTweetsResponse> {
        fetch_tweets_and_replies(self, username, max_tweets, cursor).await
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
        &mut self,
        user_id: &str,
        max_tweets: i32,
        cursor: Option<&str>,
    ) -> Result<V2QueryTweetsResponse> {
        fetch_tweets_and_replies_by_user_id(self, user_id, max_tweets, cursor).await
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
    pub async fn fetch_list_tweets(&mut self, list_id: &str, max_tweets: i32, cursor: Option<&str>) -> Result<Value> {
        fetch_list_tweets(self, list_id, max_tweets, cursor).await
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
        &mut self,
        text: &str,
        reply_to: Option<&str>,
        media_ids: Option<Vec<String>>,
    ) -> Result<Value> {
        create_long_tweet(self, text, reply_to, media_ids).await
    }
}
