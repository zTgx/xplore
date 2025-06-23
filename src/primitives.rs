pub mod tweets;
pub use tweets::*;

pub mod profile;
pub use profile::Profile;

pub mod constants;
pub use constants::BEARER_TOKEN;

mod rel;
pub use rel::*;

mod auth;
pub use auth::*;

pub mod endpoints;

use crate::error::XploreError;
pub type Result<T> = std::result::Result<T, XploreError>;
