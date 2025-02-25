use auth::user_auth::TwitterAuth;
use reqwest::Client;

pub mod api;
pub mod auth;
pub mod error;
pub mod primitives;
pub mod relationships;
pub mod impls;
pub mod search;
pub mod timeline;
pub mod tweets;

pub struct XploreX {
    pub client: Client,
    pub auth: Box<dyn TwitterAuth + Send + Sync>,
}
