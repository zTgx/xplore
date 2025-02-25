use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub username: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub protected: bool,
    pub verified: bool,
    pub followers_count: i32,
    pub following_count: i32,
    pub tweets_count: i32,
    pub listed_count: i32,
    pub created_at: DateTime<Utc>,
    pub profile_image_url: Option<String>,
    pub profile_banner_url: Option<String>,
    pub pinned_tweet_id: Option<String>,
    pub is_blue_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyUserRaw {
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub entities: Option<UserEntitiesRaw>,
    pub favourites_count: Option<i32>,
    pub followers_count: Option<i32>,
    pub friends_count: Option<i32>,
    pub media_count: Option<i32>,
    pub statuses_count: Option<i32>,
    pub id_str: Option<String>,
    pub listed_count: Option<i32>,
    pub name: Option<String>,
    pub location: String,
    pub geo_enabled: Option<bool>,
    pub pinned_tweet_ids_str: Option<Vec<String>>,
    pub profile_background_color: Option<String>,
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: Option<String>,
    pub protected: Option<bool>,
    pub screen_name: Option<String>,
    pub verified: Option<bool>,
    pub has_custom_timelines: Option<bool>,
    pub has_extended_profile: Option<bool>,
    pub url: Option<String>,
    pub can_dm: Option<bool>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntitiesRaw {
    pub url: Option<UserUrlEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUrlEntity {
    pub urls: Option<Vec<ExpandedUrl>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandedUrl {
    pub expanded_url: Option<String>,
}

lazy_static! {
    pub static ref ID_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResults {
    pub result: UserResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__typename")]
pub enum UserResult {
    User(UserData),
    UserUnavailable(UserUnavailable),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: Option<serde_json::Value>,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: LegacyUserRaw,
    pub smart_blocked_by: bool,
    pub smart_blocking: bool,
    pub legacy_extended_profile: Option<serde_json::Value>,
    pub is_profile_translatable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnavailable {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRaw {
    pub data: UserRawData,
    pub errors: Option<Vec<TwitterApiErrorRaw>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRawData {
    pub user: UserRawUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRawUser {
    pub result: UserRawResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRawResult {
    pub rest_id: Option<String>,
    pub is_blue_verified: Option<bool>,
    pub legacy: LegacyUserRaw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterApiErrorRaw {
    pub message: String,
    pub code: i32,
}
