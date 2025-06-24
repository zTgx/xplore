use {
    crate::{
        search::SearchMode,
        timeline::v1::{QueryProfilesResponse, QueryTweetsResponse},
        primitives::Result,
    },
    async_trait::async_trait,
};

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