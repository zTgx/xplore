use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tweet {
    pub ext_views: Option<i32>,
    pub created_at: Option<String>,
    pub bookmark_count: Option<i32>,
    pub conversation_id: Option<String>,
    pub hashtags: Vec<String>,
    pub html: Option<String>,
    pub id: Option<String>,
    pub in_reply_to_status: Option<Box<Tweet>>,
    pub in_reply_to_status_id: Option<String>,
    pub is_quoted: Option<bool>,
    pub is_pin: Option<bool>,
    pub is_reply: Option<bool>,
    pub is_retweet: Option<bool>,
    pub is_self_thread: Option<bool>,
    pub likes: Option<i32>,
    pub name: Option<String>,
    pub mentions: Vec<Mention>,
    pub permanent_url: Option<String>,
    pub photos: Vec<Photo>,
    pub place: Option<PlaceRaw>,
    pub quoted_status: Option<Box<Tweet>>,
    pub quoted_status_id: Option<String>,
    pub replies: Option<i32>,
    pub retweets: Option<i32>,
    pub retweeted_status: Option<Box<Tweet>>,
    pub retweeted_status_id: Option<String>,
    pub text: Option<String>,
    pub thread: Vec<Tweet>,
    pub time_parsed: Option<DateTime<Utc>>,
    pub timestamp: Option<i64>,
    pub urls: Vec<String>,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub videos: Vec<Video>,
    pub views: Option<i32>,
    pub sensitive_content: Option<bool>,
    pub poll: Option<PollV2>,
    pub quote_count: Option<i32>,
    pub reply_count: Option<i32>,
    pub retweet_count: Option<i32>,
    pub screen_name: Option<String>,
    pub thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mention {
    pub id: String,
    pub username: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id: String,
    pub url: String,
    pub alt_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub preview: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceRaw {
    pub id: Option<String>,
    pub place_type: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub bounding_box: Option<BoundingBox>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub coordinates: Option<Vec<Vec<Vec<f64>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollV2 {
    pub id: Option<String>,
    pub end_datetime: Option<String>,
    pub voting_status: Option<String>,
    pub options: Vec<PollOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    pub position: Option<i32>,
    pub label: String,
    pub votes: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetResponse {
    data: TweetData,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetData {
    create_tweet: TweetCreateResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetCreateResult {
    tweet_results: TweetResultWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetResultWrapper {
    result: TweetResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetResult {
    core: TweetCore,
    edit_control: TweetEditControl,
    is_translatable: bool,
    legacy: TweetLegacy,
    rest_id: String,
    source: String,
    unmention_data: serde_json::Value,
    unmention_info: serde_json::Value,
    views: TweetViews,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetCore {
    user_results: UserResultWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserResultWrapper {
    result: UserResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserResult {
    __typename: String,
    affiliates_highlighted_label: serde_json::Value,
    has_graduated_access: bool,
    id: String,
    is_blue_verified: bool,
    legacy: UserLegacy,
    profile_image_shape: String,
    rest_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserLegacy {
    can_dm: bool,
    can_media_tag: bool,
    created_at: String,
    default_profile: bool,
    default_profile_image: bool,
    description: String,
    entities: UserEntities,
    fast_followers_count: u64,
    favourites_count: u64,
    followers_count: u64,
    friends_count: u64,
    has_custom_timelines: bool,
    is_translator: bool,
    listed_count: u64,
    location: String,
    media_count: u64,
    name: String,
    needs_phone_verification: bool,
    normal_followers_count: u64,
    pinned_tweet_ids_str: Vec<String>,
    possibly_sensitive: bool,
    profile_image_url_https: String,
    profile_interstitial_type: String,
    screen_name: String,
    statuses_count: u64,
    translator_type: String,
    verified: bool,
    want_retweets: bool,
    withheld_in_countries: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserEntities {
    description: UserDescriptionEntities,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserDescriptionEntities {
    urls: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetEditControl {
    edit_tweet_ids: Vec<String>,
    editable_until_msecs: String,
    edits_remaining: String,
    is_edit_eligible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetLegacy {
    bookmark_count: u64,
    bookmarked: bool,
    conversation_id_str: String,
    created_at: String,
    display_text_range: Vec<u64>,
    entities: TweetEntities,
    favorite_count: u64,
    favorited: bool,
    full_text: String,
    id_str: String,
    is_quote_status: bool,
    lang: String,
    quote_count: u64,
    reply_count: u64,
    retweet_count: u64,
    retweeted: bool,
    user_id_str: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetEntities {
    hashtags: Vec<serde_json::Value>,
    symbols: Vec<serde_json::Value>,
    urls: Vec<serde_json::Value>,
    user_mentions: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TweetViews {
    state: String,
}
