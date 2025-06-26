use {
    crate::core::models::{profile::Profile, Result},
    async_trait::async_trait,
};

#[async_trait]
pub trait IProfile {
    async fn get_profile(&self, screen_name: &str) -> Result<Profile>;
    async fn get_user_id(&self, screen_name: &str) -> Result<String>;
}
