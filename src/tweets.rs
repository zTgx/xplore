use {
    crate::{
        api,
        endpoints::Endpoints,
        timeline_v2::{
            parse_threaded_conversation, parse_timeline_tweets_v2, QueryTweetsResponse, ThreadedConversation,
        },
        Result, Xplore, XploreError,
    },
    chrono::{DateTime, Utc},
    reqwest::Method,
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
};

pub const DEFAULT_EXPANSIONS: &[&str] = &[
    "attachments.poll_ids",
    "attachments.media_keys",
    "author_id",
    "referenced_tweets.id",
    "in_reply_to_user_id",
    "edit_history_tweet_ids",
    "geo.place_id",
    "entities.mentions.username",
    "referenced_tweets.id.author_id",
];

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

/// Retweet
#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRetweetResponse {
    pub data: TweetRetweetData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRetweetData {
    pub create_retweet: TweetRetweetCreateResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRetweetCreateResult {
    pub retweet_results: TweetRetweetResultWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRetweetResultWrapper {
    pub result: TweetRetweetResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRetweetResult {
    pub legacy: TweetRetweetLegacy,
    pub rest_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRetweetLegacy {
    pub full_text: String,
}

pub async fn post_tweet(
    xplore: &mut Xplore,
    text: &str,
    reply_to: Option<&str>,
    media_data: Option<Vec<(Vec<u8>, String)>>,
) -> Result<Value> {
    create_tweet_request(xplore, text, reply_to, media_data).await
}

pub async fn read_tweet(xplore: &mut Xplore, tweet_id: &str) -> Result<Tweet> {
    get_tweet(xplore, tweet_id).await
}

pub async fn retweet(xplore: &mut Xplore, tweet_id: &str) -> Result<TweetRetweetResponse> {
    let value = retweet_(xplore, tweet_id).await?;
    let res = serde_json::from_value(value)?;

    Ok(res)
}

pub async fn like_tweet(xplore: &mut Xplore, tweet_id: &str) -> Result<Value> {
    let value = like_tweet_(xplore, tweet_id).await?;
    Ok(value)
}

pub async fn get_user_tweets(xplore: &mut Xplore, user_id: &str, limit: usize) -> Result<Vec<Tweet>> {
    let url = format!("https://api.twitter.com/2/users/{}/tweets", user_id);
    let body = json!({
        "max_results": limit,
        "tweet.fields": "created_at,author_id,conversation_id,public_metrics"
    });

    let (v, _) = api::send_request::<Vec<Tweet>>(&mut xplore.auth, &url, Method::GET, Some(body)).await?;
    Ok(v)
}

pub async fn send_quote_tweet(
    xplore: &mut Xplore,
    text: &str,
    quoted_tweet_id: &str,
    media_data: Option<Vec<(Vec<u8>, String)>>,
) -> Result<Value> {
    create_quote_tweet(xplore, text, quoted_tweet_id, media_data).await
}

pub async fn fetch_tweets_and_replies(
    xplore: &mut Xplore,
    username: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<QueryTweetsResponse> {
    fetch_tweets_and_replies_(xplore, username, max_tweets, cursor).await
}

pub async fn fetch_tweets_and_replies_by_user_id(
    xplore: &mut Xplore,
    user_id: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<QueryTweetsResponse> {
    fetch_tweets_and_replies_by_user_id_(xplore, user_id, max_tweets, cursor).await
}

pub async fn fetch_list_tweets(
    xplore: &mut Xplore,
    list_id: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<Value> {
    fetch_list_tweets_(xplore, list_id, max_tweets, cursor).await
}

pub async fn create_long_tweet(
    xplore: &mut Xplore,
    text: &str,
    reply_to: Option<&str>,
    media_ids: Option<Vec<String>>,
) -> Result<Value> {
    create_long_tweet_(xplore, text, reply_to, media_ids).await
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///

pub async fn fetch_tweets(xplore: &mut Xplore, user_id: &str, max_tweets: i32, cursor: Option<&str>) -> Result<Value> {
    let mut variables = json!({
        "userId": user_id,
        "count": max_tweets.min(200),
        "includePromotedContent": false
    });

    if let Some(cursor_val) = cursor {
        variables["cursor"] = json!(cursor_val);
    }

    let url = "https://twitter.com/i/api/graphql/YNXM2DGuE2Sff6a2JD3Ztw/UserTweets";
    let method = Method::GET;
    let body = Some(json!({
        "variables": variables,
        "features": get_default_features()
    }));
    let (value, _) = api::send_request(&mut xplore.auth, url, method, body).await?;

    Ok(value)
}

pub async fn fetch_tweets_and_replies_(
    xplore: &mut Xplore,
    username: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<QueryTweetsResponse> {
    let user_id = xplore.get_user_id(username).await?;

    let endpoint = Endpoints::user_tweets_and_replies(&user_id, max_tweets.min(40), cursor);
    let url = &endpoint.to_request_url();
    let (value, _) = api::send_request(&mut xplore.auth, url, Method::GET, None).await?;

    let parsed_response = parse_timeline_tweets_v2(&value);
    Ok(parsed_response)
}

pub async fn fetch_tweets_and_replies_by_user_id_(
    xplore: &mut Xplore,
    user_id: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<QueryTweetsResponse> {
    let endpoint = Endpoints::user_tweets_and_replies(user_id, max_tweets.min(40), cursor);
    let url = &endpoint.to_request_url();
    let method = Method::GET;

    let (value, _headers) = api::send_request(&mut xplore.auth, url, method, None).await?;

    let parsed_response = parse_timeline_tweets_v2(&value);
    Ok(parsed_response)
}

pub async fn fetch_list_tweets_(
    xplore: &mut Xplore,
    list_id: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<Value> {
    let mut variables = json!({
        "listId": list_id,
        "count": max_tweets.min(200)
    });

    if let Some(cursor_val) = cursor {
        variables["cursor"] = json!(cursor_val);
    }

    let url = "https://twitter.com/i/api/graphql/LFKj1wqHNTsEJ4Oq7TzaNA/ListLatestTweetsTimeline";
    let body = Some(json!({
        "variables": variables,
        "features": get_default_features()
    }));

    let (value, _) = api::send_request(&mut xplore.auth, url, Method::GET, body).await?;

    Ok(value)
}

pub async fn create_quote_tweet(
    xplore: &mut Xplore,
    text: &str,
    quoted_tweet_id: &str,
    media_data: Option<Vec<(Vec<u8>, String)>>,
) -> Result<Value> {
    let mut variables = json!({
        "tweet_text": text,
        "dark_request": false,
        "attachment_url": format!("https://twitter.com/twitter/status/{}", quoted_tweet_id),
        "media": {
            "media_entities": [],
            "possibly_sensitive": false
        },
        "semantic_annotation_ids": []
    });

    if let Some(media_files) = media_data {
        let mut media_entities = Vec::new();

        for (file_data, media_type) in media_files {
            let media_id = upload_media(xplore, file_data, &media_type).await?;
            media_entities.push(json!({
                "media_id": media_id,
                "tagged_users": []
            }));
        }

        variables["media"]["media_entities"] = json!(media_entities);
    }

    let url = "https://twitter.com/i/api/graphql/a1p9RWpkYKBjWv_I3WzS-A/CreateTweet";
    let body = Some(json!({
        "variables": variables,
        "features": create_quote_tweet_features()
    }));
    let (v, _) = api::send_request(&mut xplore.auth, url, Method::POST, body).await?;

    Ok(v)
}

pub async fn like_tweet_(xplore: &mut Xplore, tweet_id: &str) -> Result<Value> {
    let url = "https://twitter.com/i/api/graphql/lI07N6Otwv1PhnEgXILM7A/FavoriteTweet";
    let body = Some(json!({
        "variables": {
            "tweet_id": tweet_id
        }
    }));

    let (value, _) = api::send_request(&mut xplore.auth, url, Method::POST, body).await?;
    Ok(value)
}

pub async fn retweet_(xplore: &mut Xplore, tweet_id: &str) -> Result<Value> {
    let url = "https://twitter.com/i/api/graphql/ojPdsZsimiJrUGLR1sjUtA/CreateRetweet";
    let body = Some(json!({
        "variables": {
            "tweet_id": tweet_id,
            "dark_request": false
        }
    }));
    let (value, _) = api::send_request(&mut xplore.auth, url, Method::POST, body).await?;
    Ok(value)
}

pub async fn create_long_tweet_(
    xplore: &mut Xplore,
    text: &str,
    reply_to: Option<&str>,
    media_ids: Option<Vec<String>>,
) -> Result<Value> {
    let mut variables = json!({
        "tweet_text": text,
        "dark_request": false,
        "media": {
            "media_entities": [],
            "possibly_sensitive": false
        },
        "semantic_annotation_ids": []
    });

    if let Some(reply_id) = reply_to {
        variables["reply"] = json!({
            "in_reply_to_tweet_id": reply_id
        });
    }

    if let Some(media) = media_ids {
        variables["media"]["media_entities"] = json!(media
            .iter()
            .map(|id| json!({
                "media_id": id,
                "tagged_users": []
            }))
            .collect::<Vec<_>>());
    }

    let url = "https://twitter.com/i/api/graphql/YNXM2DGuE2Sff6a2JD3Ztw/CreateNoteTweet";
    let body = Some(json!({
        "variables": variables,
        "features": get_long_tweet_features()
    }));
    let (value, _) = api::send_request(&mut xplore.auth, url, Method::POST, body).await?;

    Ok(value)
}

pub async fn fetch_liked_tweets(
    xplore: &mut Xplore,
    user_id: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<Value> {
    let mut variables = json!({
        "userId": user_id,
        "count": max_tweets.min(200),
        "includePromotedContent": false
    });

    if let Some(cursor_val) = cursor {
        variables["cursor"] = json!(cursor_val);
    }

    let url = "https://twitter.com/i/api/graphql/YlkSUg4Czo2Zx7yRqpwDow/Likes";
    let body = Some(json!({
        "variables": variables,
        "features": get_default_features()
    }));
    let (value, _) = api::send_request(&mut xplore.auth, url, Method::POST, body).await?;
    Ok(value)
}

pub async fn upload_media(xplore: &mut Xplore, file_data: Vec<u8>, media_type: &str) -> Result<String> {
    let upload_url = "https://upload.twitter.com/1.1/media/upload.json";

    // Check if media is video
    let is_video = media_type.starts_with("video/");

    if is_video {
        // Handle video upload using chunked upload
        upload_video_in_chunks(xplore, file_data, media_type).await
    } else {
        // Handle image upload directly
        let form = reqwest::multipart::Form::new().part("media", reqwest::multipart::Part::bytes(file_data));

        let (response, _) = api::request_multipart::<Value>(&mut xplore.auth, upload_url, form).await?;

        response["media_id_string"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| XploreError::Api("Failed to get media_id".into()))
    }
}

async fn upload_video_in_chunks(xplore: &mut Xplore, file_data: Vec<u8>, media_type: &str) -> Result<String> {
    let upload_url = "https://upload.twitter.com/1.1/media/upload.json";

    let body = Some(json!({
        "command": "INIT",
        "total_bytes": file_data.len(),
        "media_type": media_type
    }));
    let (init_response, _) = api::send_request::<Value>(&mut xplore.auth, upload_url, Method::POST, body).await?;

    let media_id = init_response["media_id_string"]
        .as_str()
        .ok_or_else(|| XploreError::Api("Failed to get media_id".into()))?
        .to_string();

    // APPEND command - upload in chunks
    let chunk_size = 5 * 1024 * 1024; // 5MB chunks

    for (segment_index, chunk) in file_data.chunks(chunk_size).enumerate() {
        let form = reqwest::multipart::Form::new()
            .text("command", "APPEND")
            .text("media_id", media_id.clone())
            .text("segment_index", segment_index.to_string())
            .part("media", reqwest::multipart::Part::bytes(chunk.to_vec()));

        let _ = api::request_multipart::<Value>(&mut xplore.auth, upload_url, form).await?;
    }

    // FINALIZE command
    let (finalize_response, _) = api::send_request::<Value>(
        &mut xplore.auth,
        &format!("{}?command=FINALIZE&media_id={}", upload_url, media_id),
        Method::POST,
        None,
    )
    .await?;

    // Check processing status for videos
    if finalize_response.get("processing_info").is_some() {
        check_upload_status(xplore, &media_id).await?;
    }

    Ok(media_id)
}

async fn check_upload_status(xplore: &mut Xplore, media_id: &str) -> Result<()> {
    let upload_url = "https://upload.twitter.com/1.1/media/upload.json";

    for _ in 0..20 {
        // Maximum 20 attempts
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Wait 5 seconds

        let url = &format!("{}?command=STATUS&media_id={}", upload_url, media_id);
        let method = Method::GET;
        let body = None;
        let (status_response, _) = api::send_request::<Value>(&mut xplore.auth, url, method, body).await?;

        if let Some(processing_info) = status_response.get("processing_info") {
            match processing_info["state"].as_str() {
                Some("succeeded") => return Ok(()),
                Some("failed") => return Err(XploreError::Api("Video processing failed".into())),
                _ => continue,
            }
        }
    }

    Err(XploreError::Api("Video processing timeout".into()))
}

pub async fn get_tweet(xplore: &mut Xplore, id: &str) -> Result<Tweet> {
    let tweet_detail_request = Endpoints::tweet_detail(id);
    let url = tweet_detail_request.to_request_url();

    let (response, _) = api::send_request::<Value>(&mut xplore.auth, &url, Method::GET, None).await?;
    let data = response.clone();
    let conversation: ThreadedConversation = serde_json::from_value(data)?;
    let tweets = parse_threaded_conversation(&conversation);
    tweets.into_iter().next().ok_or_else(|| XploreError::Api("No tweets found".into()))
}

fn create_tweet_features() -> Value {
    json!({
        "interactive_text_enabled": true,
        "longform_notetweets_inline_media_enabled": false,
        "responsive_web_text_conversations_enabled": false,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": false,
        "vibe_api_enabled": false,
        "rweb_lists_timeline_redesign_enabled": true,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "verified_phone_label_enabled": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "tweetypie_unmention_optimization_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "view_counts_everywhere_api_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "tweet_awards_web_tipping_enabled": false,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "standardized_nudges_misinfo": true,
        "longform_notetweets_rich_text_read_enabled": true,
        "responsive_web_enhance_cards_enabled": false,
        "subscriptions_verification_info_enabled": true,
        "subscriptions_verification_info_reason_enabled": true,
        "subscriptions_verification_info_verified_since_enabled": true,
        "super_follow_badge_privacy_enabled": false,
        "super_follow_exclusive_tweet_notifications_enabled": false,
        "super_follow_tweet_api_enabled": false,
        "super_follow_user_api_enabled": false,
        "android_graphql_skip_api_media_color_palette": false,
        "creator_subscriptions_subscription_count_enabled": false,
        "blue_business_profile_image_shape_enabled": false,
        "unified_cards_ad_metadata_container_dynamic_card_content_query_enabled": false,
        "rweb_video_timestamps_enabled": false,
        "c9s_tweet_anatomy_moderator_badge_enabled": false,
        "responsive_web_twitter_article_tweet_consumption_enabled": false
    })
}

fn get_default_features() -> Value {
    json!({
        "interactive_text_enabled": true,
        "longform_notetweets_inline_media_enabled": false,
        "responsive_web_text_conversations_enabled": false,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": false,
        "vibe_api_enabled": false,
        "rweb_lists_timeline_redesign_enabled": true,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "verified_phone_label_enabled": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "tweetypie_unmention_optimization_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "view_counts_everywhere_api_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "tweet_awards_web_tipping_enabled": false,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "standardized_nudges_misinfo": true,
        "longform_notetweets_rich_text_read_enabled": true,
        "responsive_web_enhance_cards_enabled": false,
        "subscriptions_verification_info_enabled": true,
        "subscriptions_verification_info_reason_enabled": true,
        "subscriptions_verification_info_verified_since_enabled": true,
        "super_follow_badge_privacy_enabled": false,
        "super_follow_exclusive_tweet_notifications_enabled": false,
        "super_follow_tweet_api_enabled": false,
        "super_follow_user_api_enabled": false,
        "android_graphql_skip_api_media_color_palette": false,
        "creator_subscriptions_subscription_count_enabled": false,
        "blue_business_profile_image_shape_enabled": false,
        "unified_cards_ad_metadata_container_dynamic_card_content_query_enabled": false,
        "rweb_video_timestamps_enabled": true,
        "c9s_tweet_anatomy_moderator_badge_enabled": true,
        "responsive_web_twitter_article_tweet_consumption_enabled": false,
        "creator_subscriptions_quote_tweet_preview_enabled": false,
        "profile_label_improvements_pcf_label_in_post_enabled": false,
        "rweb_tipjar_consumption_enabled": true,
        "articles_preview_enabled": true
    })
}

// Helper function for long tweet features
fn get_long_tweet_features() -> Value {
    json!({
        "premium_content_api_read_enabled": false,
        "communities_web_enable_tweet_community_results_fetch": true,
        "c9s_tweet_anatomy_moderator_badge_enabled": true,
        "responsive_web_grok_analyze_button_fetch_trends_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "view_counts_everywhere_api_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "responsive_web_twitter_article_tweet_consumption_enabled": true,
        "tweet_awards_web_tipping_enabled": false,
        "longform_notetweets_rich_text_read_enabled": true,
        "longform_notetweets_inline_media_enabled": true,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "verified_phone_label_enabled": false,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "standardized_nudges_misinfo": true,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_enhance_cards_enabled": false
    })
}

pub async fn create_tweet_request(
    xplore: &mut Xplore,
    text: &str,
    reply_to: Option<&str>,
    media_data: Option<Vec<(Vec<u8>, String)>>,
) -> Result<Value> {
    // Prepare variables
    let mut variables = json!({
        "tweet_text": text,
        "dark_request": false,
        "media": {
            "media_entities": [],
            "possibly_sensitive": false
        },
        "semantic_annotation_ids": []
    });

    // Add reply information if provided
    if let Some(reply_id) = reply_to {
        variables["reply"] = json!({
            "in_reply_to_tweet_id": reply_id
        });
    }

    // Handle media uploads if provided
    if let Some(media_files) = media_data {
        let mut media_entities = Vec::new();

        // Upload each media file and collect media IDs
        for (file_data, media_type) in media_files {
            let media_id = upload_media(xplore, file_data, &media_type).await?;
            media_entities.push(json!({
                "media_id": media_id,
                "tagged_users": []
            }));
        }

        variables["media"]["media_entities"] = json!(media_entities);
    }
    let features = create_tweet_features();

    let url = "https://twitter.com/i/api/graphql/a1p9RWpkYKBjWv_I3WzS-A/CreateTweet";
    let body = Some(json!({
        "variables": variables,
        "features": features,
        "fieldToggles": {}
    }));
    let (value, _) = api::send_request(&mut xplore.auth, url, Method::POST, body).await?;
    Ok(value)
}

fn create_quote_tweet_features() -> Value {
    json!({
        "interactive_text_enabled": true,
        "longform_notetweets_inline_media_enabled": false,
        "responsive_web_text_conversations_enabled": false,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": false,
        "vibe_api_enabled": false,
        "rweb_lists_timeline_redesign_enabled": true,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "verified_phone_label_enabled": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "tweetypie_unmention_optimization_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "view_counts_everywhere_api_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "tweet_awards_web_tipping_enabled": false,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "standardized_nudges_misinfo": true,
        "longform_notetweets_rich_text_read_enabled": true,
        "responsive_web_enhance_cards_enabled": false,
        "subscriptions_verification_info_enabled": true,
        "subscriptions_verification_info_reason_enabled": true,
        "subscriptions_verification_info_verified_since_enabled": true,
        "super_follow_badge_privacy_enabled": false,
        "super_follow_exclusive_tweet_notifications_enabled": false,
        "super_follow_tweet_api_enabled": false,
        "super_follow_user_api_enabled": false,
        "android_graphql_skip_api_media_color_palette": false,
        "creator_subscriptions_subscription_count_enabled": false,
        "blue_business_profile_image_shape_enabled": false,
        "unified_cards_ad_metadata_container_dynamic_card_content_query_enabled": false,
        "rweb_video_timestamps_enabled": true,
        "c9s_tweet_anatomy_moderator_badge_enabled": true,
        "responsive_web_twitter_article_tweet_consumption_enabled": false
    })
}

pub async fn fetch_user_tweets(
    xplore: &mut Xplore,
    user_id: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<QueryTweetsResponse> {
    let endpoint = Endpoints::user_tweets(user_id, max_tweets.min(200), cursor);
    let url = &endpoint.to_request_url();

    let (value, _) = api::send_request(&mut xplore.auth, url, Method::GET, None).await?;

    let parsed_response = parse_timeline_tweets_v2(&value);
    Ok(parsed_response)
}
