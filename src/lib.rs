//! Xplore - A Twitter API client for Rust
//!
//! This crate provides a convenient way to interact with Twitter's API.

pub mod api;
pub mod core;
pub mod error;
pub mod services;
pub mod utils;

// pub use crate::{
//     core::Xplore,
//     services::{IProfile, ITweet, ISearch},
//     primitives::{Result, Profile, Tweet, TweetRetweetResponse},
//     search::SearchMode,
//     timeline::v1::{QueryProfilesResponse, QueryTweetsResponse},
//     timeline::v2::{QueryTweetsResponse as V2QueryTweetsResponse},
// };
