use {
    crate::core::models::{profile::Profile, Result},
    async_trait::async_trait,
};

#[async_trait]
pub trait IRel {
    async fn following(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)>;

    async fn followers(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)>;

    async fn follow(&self, username: &str) -> Result<()>;
    async fn unfollow(&self, username: &str) -> Result<()>;
}
