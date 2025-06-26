use {
    crate::{
        api::{profile::IProfile, rel::IRel},
        core::{
            client::Xplore,
            models::{profile::Profile, Result},
        },
        utils::rel as rel_utils,
    },
    async_trait::async_trait,
    serde_json::Value,
};

#[async_trait]
impl IRel for Xplore {
    async fn following(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        let response = rel_utils::fetch_profile_following(self, user_id, count, cursor).await?;
        Ok((response.profiles, response.next))
    }

    async fn followers(
        &self,
        user_id: &str,
        count: i32,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        let response = rel_utils::fetch_profile_followers(self, user_id, count, cursor).await?;
        Ok((response.profiles, response.next))
    }

    async fn follow(&self, username: &str) -> Result<()> {
        let user_id = self.get_user_id(username).await?;

        let url = "https://api.twitter.com/1.1/friendships/create.json";

        let form = vec![
            ("include_profile_interstitial_type".to_string(), "1".to_string()),
            ("skip_status".to_string(), "true".to_string()),
            ("user_id".to_string(), user_id),
        ];

        let _ = self.inner.rpc.request_form::<Value>(url, username, form).await?;

        Ok(())
    }

    async fn unfollow(&self, username: &str) -> Result<()> {
        let user_id = self.get_user_id(username).await?;

        let url = "https://api.twitter.com/1.1/friendships/destroy.json";

        let form = vec![
            ("include_profile_interstitial_type".to_string(), "1".to_string()),
            ("skip_status".to_string(), "true".to_string()),
            ("user_id".to_string(), user_id),
        ];

        let (_, _) = self.inner.rpc.request_form::<Value>(url, username, form).await?;

        Ok(())
    }
}
