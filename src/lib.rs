//! Xplore - A Twitter API client for Rust
//!
//! This crate provides a convenient way to interact with Twitter's API.

pub mod api;
pub mod core;
pub mod services;
pub mod utils;

pub use crate::{
    api::{home::IHome, profile::IProfile, rel::IRel, search::ISearch, tweet::ITweet},
    core::{client::Xplore, error::XploreError, models::*},
};
