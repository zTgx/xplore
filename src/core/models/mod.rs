pub mod auth;
pub mod constants;
pub mod endpoints;
pub mod home;
pub mod profile;
pub mod rel;
pub mod search;
pub mod timeline_v1;
pub mod timeline_v2;
pub mod tweets;

use crate::error::XploreError;
pub type Result<T> = std::result::Result<T, XploreError>;
