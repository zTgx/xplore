use auth::user_auth::{TwitterAuth, TwitterUserAuth};
use dotenv::dotenv;
use primitives::BEARER_TOKEN;
use reqwest::Client;
use std::env;
use std::time::Duration;

pub mod api;
pub mod auth;
pub mod error;
pub mod impls;
pub mod primitives;
pub mod relationships;
pub mod search;
pub mod timeline;
pub mod tweets;

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
