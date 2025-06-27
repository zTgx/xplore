use {
    crate::{core::models::Result, utils::home, Xplore},
    serde_json::Value,
};

pub async fn get_home_timeline(xplore: &Xplore, count: i32, seen_tweet_ids: Vec<String>) -> Result<Vec<Value>> {
    home::fetch_home_timeline(xplore, count, seen_tweet_ids).await
}
