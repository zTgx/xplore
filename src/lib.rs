use async_trait::async_trait;
use auth::user_auth::TwitterUserAuth;
use primitives::Profile;
use reqwest::Client;
use crate::error::Result;

pub mod api;
pub mod auth;
pub mod error;
pub mod impls;
pub mod primitives;
pub mod relationships;
pub mod search;
pub mod timeline;
pub mod tweets;

#[async_trait]
pub trait IProfile {
    async fn get_profile_by_screen_name(&self, screen_name: &str) -> Result<Profile>;
    async fn get_screen_name_by_user_id(&self, user_id: &str) -> Result<String>;
    async fn get_user_id_by_screen_name(&self, screen_name: &str) -> Result<String>;
}

pub struct XploreX {
    pub client: Client,
    pub auth: TwitterUserAuth,
}

impl XploreX {
    pub async fn new(cookie: &str) -> Result<Self> {
        let client = xplore_utils::client()?;
        let auth = xplore_utils::make_auth(cookie).await?;

        Ok(XploreX {
            client,
            auth,
        })
    }
}

mod xplore_utils {
    use reqwest::Client;
    use std::time::Duration;
    use crate::{auth::user_auth::TwitterUserAuth, error::{Result, TwitterError}};

    pub fn client() -> Result<Client> {
        Client::builder()
        .timeout(Duration::from_secs(30))
        .cookie_store(true)
        .build()
        .map_err(|e| {
            TwitterError::Network(e)
        })
    }

    pub async fn make_auth(cookie: &str) -> Result<TwitterUserAuth> {
        let mut auth = TwitterUserAuth::new()
            .await?;

        auth.set_from_cookie_string(cookie)
            .await?;

        Ok(auth)
    }
}