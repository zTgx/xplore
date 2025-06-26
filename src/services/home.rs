use {
    crate::{
        api::home::IHome,
        core::{client::Xplore, models::Result},
        utils::home,
    },
    async_trait::async_trait,
    serde_json::Value,
};

#[async_trait]
impl IHome for Xplore {
    async fn get_home_timeline(&self, count: i32, seen_tweet_ids: Vec<String>) -> Result<Vec<Value>> {
        home::fetch_home_timeline(self, count, seen_tweet_ids).await
    }
}
