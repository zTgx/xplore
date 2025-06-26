use {crate::core::models::Result, async_trait::async_trait, serde_json::Value};

#[async_trait]
pub trait IHome {
    async fn get_home_timeline(&self, count: i32, seen_tweet_ids: Vec<String>) -> Result<Vec<Value>>;
}
