use {
    crate::core::models::{
        search::SearchMode,
        timeline_v1::{QueryProfilesResponse, QueryTweetsResponse},
        Result,
    },
    async_trait::async_trait,
};

#[async_trait]
pub trait ISearch {
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
    async fn search_tweets(
        &self,
        query: &str,
        max_tweets: i32,
        search_mode: SearchMode,
        cursor: Option<String>,
    ) -> Result<QueryTweetsResponse>;

    ///! Searches for user profiles based on a query string.
    /// # Arguments
    /// * `query` - The search query string to find user profiles.
    /// * `max_profiles` - The maximum number of profiles to return.
    /// * `cursor` - An optional cursor for pagination.
    /// # Returns
    /// * `Result<QueryProfilesResponse>` - A result containing the response with user profiles if successful, or an error if not.
    /// # Errors
    /// Returns an error if the search fails, such as if the query is invalid or if there is a network issue.
    async fn search_profiles(
        &self,
        query: &str,
        max_profiles: i32,
        cursor: Option<String>,
    ) -> Result<QueryProfilesResponse>;
}
