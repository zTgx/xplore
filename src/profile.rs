use {
    crate::{api, auth::UserAuth, Result, XploreError},
    chrono::{DateTime, Utc},
    lazy_static::lazy_static,
    reqwest::Method,
    serde::{Deserialize, Serialize},
    serde_json::json,
    std::collections::HashMap,
    std::sync::Arc,
    tokio::sync::Mutex,
};

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
    pub followers_count: u32,
    pub following_count: u32,
    pub tweets_count: u32,
    pub listed_count: u32,
    pub created_at: DateTime<Utc>,
    pub profile_image_url: Option<String>,
    pub profile_banner_url: Option<String>,
    pub pinned_tweet_id: Option<Vec<String>>,
    pub is_blue_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyUserRaw {
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub entities: Option<UserEntitiesRaw>,
    pub favourites_count: Option<u32>,
    pub followers_count: Option<u32>,
    pub friends_count: Option<u32>,
    pub media_count: Option<u32>,
    pub statuses_count: Option<u32>,
    pub id_str: Option<String>,
    pub listed_count: Option<u32>,
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

impl From<(&LegacyUserRaw, Option<bool>)> for Profile {
    fn from((user, is_blue_verified): (&LegacyUserRaw, Option<bool>)) -> Self {
        let mut profile = Profile {
            id: user.user_id.clone().unwrap_or_default(),
            username: user.screen_name.clone().unwrap_or_default(),
            name: user.name.clone().unwrap_or_default(),
            description: user.description.clone(),
            location: Some(user.location.clone()),
            url: user.url.clone(),
            protected: user.protected.unwrap_or(false),
            verified: user.verified.unwrap_or(false),
            followers_count: user.followers_count.unwrap_or(0),
            following_count: user.friends_count.unwrap_or(0),
            tweets_count: user.statuses_count.unwrap_or(0),
            listed_count: user.listed_count.unwrap_or(0),
            is_blue_verified,
            created_at: user
                .created_at
                .as_ref()
                .and_then(|date_str| {
                    DateTime::parse_from_str(date_str, "%a %b %d %H:%M:%S %z %Y").ok().map(|dt| dt.with_timezone(&Utc))
                })
                .unwrap_or_else(Utc::now),
            profile_image_url: user.profile_image_url_https.as_ref().map(|url| url.replace("_normal", "")),
            profile_banner_url: user.profile_banner_url.clone(),
            pinned_tweet_id: user.pinned_tweet_ids_str.as_ref().and_then(|ids| Some(ids.clone())),
        };

        // Set website URL from entities using functional chaining
        user.entities
            .as_ref()
            .and_then(|entities| entities.url.as_ref())
            .and_then(|url_entity| url_entity.urls.as_ref())
            .and_then(|urls| urls.first())
            .and_then(|first_url| first_url.expanded_url.as_ref())
            .map(|expanded_url| profile.url = Some(expanded_url.clone()));

        if let Some(expanded_url) = user
            .entities
            .as_ref()
            .and_then(|entities| entities.url.as_ref())
            .and_then(|url_entity| url_entity.urls.as_ref())
            .and_then(|urls| urls.first())
            .and_then(|first_url| first_url.expanded_url.as_ref())
        {
            profile.url = Some(expanded_url.clone())
        }

        profile
    }
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
    pub static ref ID_CACHE: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
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

pub async fn get_profile(auth: &mut UserAuth, screen_name: &str) -> Result<Profile> {
    let body = json!({
        "variables": json!({
            "screen_name": screen_name,
            "withSafetyModeUserFields": true
        }),
        "features": json!({
            "hidden_profile_likes_enabled": false,
            "hidden_profile_subscriptions_enabled": false,
            "responsive_web_graphql_exclude_directive_enabled": true,
            "verified_phone_label_enabled": false,
            "subscriptions_verification_info_is_identity_verified_enabled": false,
            "subscriptions_verification_info_verified_since_enabled": true,
            "highlights_tweets_tab_ui_enabled": true,
            "creator_subscriptions_tweet_preview_api_enabled": true,
            "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
            "responsive_web_graphql_timeline_navigation_enabled": true
        }),
        "fieldToggles": json!({
            "withAuxiliaryUserLabels": false
        })
    });
    let (user_raw, _) = api::send_request::<UserRaw>(
        auth,
        "https://twitter.com/i/api/graphql/G3KGOASz96M-Qu0nwmGXNg/UserByScreenName",
        Method::GET,
        Some(body),
    )
    .await?;

    if let Some(errors) = user_raw.errors {
        if !errors.is_empty() {
            return Err(XploreError::Api(errors[0].message.clone()));
        }
    }

    let user_legacy = &user_raw.data.user.result.legacy;
    let rest_id = user_raw.data.user.result.rest_id.clone();
    let is_blue_verified = user_raw.data.user.result.is_blue_verified;

    let mut legacy = user_legacy.clone();
    legacy.user_id = rest_id;

    match legacy.screen_name.as_deref() {
        Some(name) if !name.is_empty() => Ok((&legacy, is_blue_verified).into()),
        _ => Err(XploreError::Api(format!("Either {} does not exist or is private.", screen_name))),
    }
}

pub async fn get_user_id(auth: &mut UserAuth, screen_name: &str) -> Result<String> {
    let cache = ID_CACHE.clone();

    let mut cache = cache.lock().await;
    if let Some(cached_id) = cache.get(screen_name) {
        return Ok(cached_id.clone());
    }

    let profile = get_profile(auth, screen_name).await?;

    let user_id = profile.id;

    cache.insert(screen_name.to_string(), user_id.clone());

    Ok(user_id)
}
