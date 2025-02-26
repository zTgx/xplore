use async_trait::async_trait;
use auth::user_auth::{TwitterAuth, TwitterUserAuth};
use dotenv::dotenv;
use primitives::{Profile, BEARER_TOKEN};
use reqwest::Client;
use std::env;
use std::time::Duration;
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
    pub auth: Box<dyn TwitterAuth + Send + Sync>,
}

impl XploreX {
    pub async fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .expect("reqwest client");

        dotenv().ok();

        let x_cookie_string = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");

        let mut auth = TwitterUserAuth::new(BEARER_TOKEN.to_string())
            .await
            .expect("X user auth");
        auth.set_from_cookie_string(&x_cookie_string)
            .await
            .expect("x cookie string");

        let auth = Box::new(auth);
        XploreX { client, auth }
    }
}
