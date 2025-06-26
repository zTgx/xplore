use {
    crate::{
        api::search::ISearch,
        core::{
            client::Xplore,
            models::{
                search::SearchMode,
                timeline_v1::{QueryProfilesResponse, QueryTweetsResponse},
                Result,
            },
        },
        utils::search as search_utils,
    },
    async_trait::async_trait,
};

#[async_trait]
impl ISearch for Xplore {
    async fn search_tweets(
        &self,
        query: &str,
        max_tweets: i32,
        search_mode: SearchMode,
        cursor: Option<String>,
    ) -> Result<QueryTweetsResponse> {
        let timeline = search_utils::get_search_timeline(self, query, max_tweets, search_mode, cursor).await?;

        Ok(search_utils::parse_search_timeline_tweets(&timeline))
    }

    async fn search_profiles(
        &self,
        query: &str,
        max_profiles: i32,
        cursor: Option<String>,
    ) -> Result<QueryProfilesResponse> {
        let timeline = search_utils::get_search_timeline(self, query, max_profiles, SearchMode::Users, cursor).await?;

        Ok(search_utils::parse_search_timeline_users(&timeline))
    }
}
