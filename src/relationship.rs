use {
    crate::{api, timeline_v1::QueryProfilesResponse, Profile, Result, Xplore, XploreError},
    chrono::{DateTime, Utc},
    reqwest::Method,
    serde::Deserialize,
    serde_json::{json, Value},
};

#[derive(Debug, Deserialize)]
pub struct HomeTimelineResponse {
    pub data: Option<HomeData>,
}

#[derive(Debug, Deserialize)]
pub struct HomeData {
    pub home: Home,
}

#[derive(Debug, Deserialize)]
pub struct Home {
    #[serde(rename = "home_timeline_urt")]
    pub home_timeline: HomeTimeline,
}

#[derive(Debug, Deserialize)]
pub struct HomeTimeline {
    pub instructions: Vec<HomeTimelineInstruction>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum HomeTimelineInstruction {
    #[serde(rename = "TimelineAddEntries")]
    AddEntries { entries: Vec<HomeTimelineEntry> },
    // Add other variants as needed
}

#[derive(Debug, Deserialize)]
pub struct HomeTimelineEntry {
    pub content: HomeEntryContent,
}

#[derive(Debug, Deserialize)]
pub struct HomeEntryContent {
    #[serde(rename = "itemContent")]
    pub item_content: Option<HomeItemContent>,
}

#[derive(Debug, Deserialize)]
pub struct HomeItemContent {
    pub tweet_results: Option<TweetResults>,
}

#[derive(Debug, Deserialize)]
pub struct TweetResults {
    pub result: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipResponse {
    pub data: Option<RelationshipData>,
    #[serde(skip)]
    pub errors: Option<Vec<XploreError>>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipData {
    pub user: UserRelationships,
}

#[derive(Debug, Deserialize)]
pub struct UserRelationships {
    pub result: UserResult,
}

#[derive(Debug, Deserialize)]
pub struct UserResult {
    pub timeline: Timeline,
    pub rest_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Timeline {
    pub timeline: TimelineData,
}

#[derive(Debug, Deserialize)]
pub struct TimelineData {
    pub instructions: Vec<TimelineInstruction>,
}

#[derive(Debug, Deserialize)]
pub struct TimelineInstruction {
    #[serde(rename = "type")]
    pub instruction_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<TimelineEntry>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<TimelineEntry>,
}

#[derive(Debug, Deserialize)]
pub struct TimelineEntry {
    #[serde(rename = "entryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<String>,

    #[serde(rename = "sortIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_index: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<EntryContent>,
}

#[derive(Debug, Deserialize)]
pub struct EntryContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cursorType")]
    pub cursor_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entryType")]
    pub entry_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub __typename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<CursorContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientEventInfo")]
    pub client_event_info: Option<Value>, // Empty object
}

#[derive(Debug, Deserialize)]
pub struct ItemContent {
    #[serde(rename = "user_results")]
    pub user_results: Option<UserResults>,

    #[serde(rename = "userDisplayType")]
    pub user_display_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "itemType")]
    pub item_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserResults {
    pub result: UserResultData,
}

#[derive(Debug, Deserialize)]
pub struct UserResultData {
    #[serde(rename = "__typename")]
    pub typename: String,

    #[serde(rename = "affiliates_highlighted_label")]
    pub affiliates_highlighted_label: Value, // Empty object

    #[serde(rename = "has_graduated_access")]
    pub has_graduated_access: bool,

    pub id: String,

    #[serde(rename = "is_blue_verified")]
    pub is_blue_verified: bool,

    #[serde(rename = "legacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<UserLegacy>,

    pub professional: Option<Professional>,

    #[serde(rename = "profile_image_shape")]
    pub profile_image_shape: String,

    #[serde(rename = "rest_id")]
    pub rest_id: String,

    #[serde(rename = "super_follow_eligible")]
    pub super_follow_eligible: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct MediaColor {
    pub r: Option<ColorPalette>,
}

#[derive(Debug, Deserialize)]
pub struct ColorPalette {
    pub ok: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Url {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlInfo>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UrlInfo {
    #[serde(rename = "expanded_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserLegacy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_count: Option<u32>,

    #[serde(rename = "id_str")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_str: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "geo_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_enabled: Option<bool>,

    #[serde(rename = "pinned_tweet_ids_str")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_tweet_ids_str: Option<Vec<String>>,

    #[serde(rename = "profile_background_color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_color: Option<String>,

    #[serde(rename = "profile_banner_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_banner_url: Option<String>,

    #[serde(rename = "profile_image_url_https")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url_https: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,

    #[serde(rename = "screen_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    #[serde(rename = "has_custom_timelines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_custom_timelines: Option<bool>,

    #[serde(rename = "has_extended_profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_extended_profile: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "can_dm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_dm: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Professional {
    pub rest_id: Option<String>,
    pub professional_type: Option<String>,
    pub category: Option<Vec<ProfessionalCategory>>,
}

#[derive(Debug, Deserialize)]
pub struct ProfessionalCategory {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CursorContent {
    pub value: String,

    #[serde(rename = "cursorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimeline {
    pub data: Option<RelationshipTimelineData>,
    pub errors: Option<Vec<XploreError>>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimelineData {
    pub user: UserData,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub result: RelationshipUserResult,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipUserResult {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,

    pub timeline: Timeline,
}

#[derive(Debug, Deserialize)]
pub struct InnerTimeline {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Instruction {
    #[serde(rename = "TimelineAddEntries")]
    AddEntries { entries: Vec<RelationshipTimelineEntry> },
    #[serde(rename = "TimelineReplaceEntry")]
    ReplaceEntry { entry: RelationshipTimelineEntry },
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimelineEntry {
    pub content: EntryContent,
    pub entry_id: String,
    pub sort_index: String,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimelineContainer {
    pub timeline: InnerTimeline,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimelineWrapper {
    pub timeline: InnerTimeline,
}

pub async fn get_home_timeline(xplore: &mut Xplore, count: i32, seen_tweet_ids: Vec<String>) -> Result<Vec<Value>> {
    fetch_home_timeline(xplore, count, seen_tweet_ids).await
}

pub async fn get_following(
    xplore: &mut Xplore,
    user_id: &str,
    count: i32,
    cursor: Option<String>,
) -> Result<(Vec<Profile>, Option<String>)> {
    let response = fetch_profile_following(xplore, user_id, count, cursor).await?;
    Ok((response.profiles, response.next))
}

pub async fn get_followers(
    xplore: &mut Xplore,
    user_id: &str,
    count: i32,
    cursor: Option<String>,
) -> Result<(Vec<Profile>, Option<String>)> {
    let response = fetch_profile_followers(xplore, user_id, count, cursor).await?;
    Ok((response.profiles, response.next))
}

///! TODO: error handling
pub async fn follow(xplore: &mut Xplore, username: &str) -> Result<()> {
    let user_id = xplore.get_user_id(username).await?;

    let url = "https://api.twitter.com/1.1/friendships/create.json";

    let form = vec![
        ("include_profile_interstitial_type".to_string(), "1".to_string()),
        ("skip_status".to_string(), "true".to_string()),
        ("user_id".to_string(), user_id),
    ];

    let _ = api::request_form::<Value>(&mut xplore.auth, url, username, form).await?;

    Ok(())
}

///! TODO: error handling
pub async fn unfollow(xplore: &mut Xplore, username: &str) -> Result<()> {
    let user_id = xplore.get_user_id(username).await?;

    let url = "https://api.twitter.com/1.1/friendships/destroy.json";

    let form = vec![
        ("include_profile_interstitial_type".to_string(), "1".to_string()),
        ("skip_status".to_string(), "true".to_string()),
        ("user_id".to_string(), user_id),
    ];

    let (_, _) = api::request_form::<Value>(&mut xplore.auth, url, username, form).await?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
pub async fn fetch_profile_following(
    xplore: &mut Xplore,
    user_id: &str,
    max_profiles: i32,
    cursor: Option<String>,
) -> Result<QueryProfilesResponse> {
    let timeline = get_following_timeline(xplore, user_id, max_profiles, cursor).await?;
    Ok(parse_relationship_timeline(&timeline))
}

pub async fn fetch_profile_followers(
    xplore: &mut Xplore,
    user_id: &str,
    max_profiles: i32,
    cursor: Option<String>,
) -> Result<QueryProfilesResponse> {
    let timeline = get_follower_timeline(xplore, user_id, max_profiles, cursor).await?;
    Ok(parse_relationship_timeline(&timeline))
}

fn parse_relationship_timeline(timeline: &RelationshipTimeline) -> QueryProfilesResponse {
    let mut profiles = Vec::new();
    let mut bottom_cursor = None;
    let mut top_cursor = None;

    if let Some(data) = &timeline.data {
        for instruction in &data.user.result.timeline.timeline.instructions {
            if instruction.instruction_type == "TimelineAddEntries"
                || instruction.instruction_type == "TimelineReplaceEntry"
            {
                // Handle case where instruction has a single entry (entry field)
                if let Some(entry_content) = &instruction.entry {
                    if let Some(content) = &entry_content.content {
                        if let Some(cursor_type) = &content.cursor_type {
                            if cursor_type == "Bottom" {
                                if let Some(value) = &content.value {
                                    bottom_cursor = Some(value.clone());
                                    continue;
                                }
                            } else if cursor_type == "Top" {
                                if let Some(value) = &content.value {
                                    top_cursor = Some(value.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }

                // Handle case where instruction has multiple entries (entries field)
                if let Some(entries) = &instruction.entries {
                    for entry in entries {
                        if let Some(item_content) = &entry.content.as_ref().and_then(|c| c.item_content.as_ref()) {
                            if item_content.user_display_type == Some("User".to_string()) {
                                if let Some(user_result_raw) = &item_content.user_results {
                                    if let Some(legacy) = &user_result_raw.result.legacy {
                                        let profile = Profile {
                                            username: legacy.screen_name.clone().unwrap_or_default(),
                                            name: legacy.name.clone().unwrap_or_default(),
                                            id: user_result_raw.result.rest_id.to_string(),
                                            description: legacy.description.clone(),
                                            location: legacy.location.clone(),
                                            url: legacy.url.clone(),
                                            protected: legacy.protected.unwrap_or_default(),
                                            verified: legacy.verified.unwrap_or_default(),
                                            followers_count: legacy.followers_count.unwrap_or_default(),
                                            following_count: legacy.friends_count.unwrap_or_default(),
                                            tweets_count: legacy.statuses_count.unwrap_or_default(),
                                            listed_count: legacy.listed_count.unwrap_or_default(),
                                            created_at: legacy
                                                .created_at
                                                .as_ref()
                                                .and_then(|date| {
                                                    DateTime::parse_from_str(date, "%a %b %d %H:%M:%S %z %Y")
                                                        .ok()
                                                        .map(|dt| dt.with_timezone(&Utc))
                                                })
                                                .unwrap_or_default(),
                                            profile_image_url: legacy.profile_image_url_https.clone(),
                                            profile_banner_url: legacy.profile_banner_url.clone(),
                                            pinned_tweet_id: legacy.pinned_tweet_ids_str.clone(),
                                            is_blue_verified: Some(user_result_raw.result.is_blue_verified),
                                        };

                                        profiles.push(profile);
                                    }
                                }
                            } else if let Some(cursor_type) =
                                &entry.content.as_ref().and_then(|c| c.cursor_type.as_ref())
                            {
                                if cursor_type.to_string() == "Bottom" {
                                    if let Some(value) = &entry.content.as_ref().and_then(|c| c.value.as_ref()) {
                                        bottom_cursor = Some(value.to_string());
                                    }
                                } else if cursor_type.to_string() == "Top" {
                                    if let Some(value) = &entry.content.as_ref().and_then(|c| c.value.as_ref()) {
                                        top_cursor = Some(value.to_string());
                                    }
                                }
                            }
                        }
                    }
                }

                // println!("Processing instruction: {:?}", instruction);
            }
        }
    }

    QueryProfilesResponse { profiles, next: bottom_cursor, previous: top_cursor }
}

pub async fn fetch_home_timeline(xplore: &mut Xplore, count: i32, seen_tweet_ids: Vec<String>) -> Result<Vec<Value>> {
    let variables = serde_json::json!({
        "count": count,
        "includePromotedContent": true,
        "latestControlAvailable": true,
        "requestContext": "launch",
        "withCommunity": true,
        "seenTweetIds": seen_tweet_ids,
    });

    let features = serde_json::json!({
        "rweb_tipjar_consumption_enabled": true,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "verified_phone_label_enabled": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "communities_web_enable_tweet_community_results_fetch": true,
        "c9s_tweet_anatomy_moderator_badge_enabled": true,
        "articles_preview_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "view_counts_everywhere_api_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "responsive_web_twitter_article_tweet_consumption_enabled": true,
        "tweet_awards_web_tipping_enabled": false,
        "creator_subscriptions_quote_tweet_preview_enabled": false,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "standardized_nudges_misinfo": true,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
        "rweb_video_timestamps_enabled": true,
        "longform_notetweets_rich_text_read_enabled": true,
        "longform_notetweets_inline_media_enabled": true,
        "responsive_web_enhance_cards_enabled": false,
    });

    let url = format!(
        "https://x.com/i/api/graphql/HJFjzBgCs16TqxewQOeLNg/HomeTimeline?variables={}&features={}",
        urlencoding::encode(&variables.to_string()),
        urlencoding::encode(&features.to_string())
    );

    let (response, _) = api::send_request::<HomeTimelineResponse>(&mut xplore.auth, &url, Method::GET, None).await?;

    let home = response.data.map(|data| data.home.home_timeline.instructions);

    let mut entries = Vec::new();

    if let Some(instructions) = home {
        for instruction in instructions {
            match instruction {
                HomeTimelineInstruction::AddEntries { entries: new_entries } => {
                    for entry in new_entries {
                        if let Some(item_content) = entry.content.item_content {
                            if let Some(tweet_results) = item_content.tweet_results {
                                if let Some(result) = tweet_results.result {
                                    entries.push(result);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(entries)
}

pub async fn get_following_timeline(
    xplore: &mut Xplore,
    user_id: &str,
    max_items: i32,
    cursor: Option<String>,
) -> Result<RelationshipTimeline> {
    let max_items = if max_items > 50 { 50 } else { max_items };

    let mut variables = json!({
        "userId": user_id,
        "count": max_items,
        "includePromotedContent": false,
    });

    let features = json!({
        "responsive_web_twitter_article_tweet_consumption_enabled": false,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
        "longform_notetweets_inline_media_enabled": true,
        "responsive_web_media_download_video_enabled": false,

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
        "unified_cards_ad_metadata_container_dynamic_card_content_query_enabled": false
    });

    if let Some(cursor_val) = cursor {
        if !cursor_val.is_empty() {
            variables["cursor"] = json!(cursor_val);
        }
    }

    let url = format!(
        "https://x.com/i/api/graphql/iSicc7LrzWGBgDPL0tM_TQ/Following?variables={}&features={}",
        urlencoding::encode(&variables.to_string()),
        urlencoding::encode(&features.to_string())
    );

    let (data, _) = api::send_request::<RelationshipTimeline>(&mut xplore.auth, &url, Method::GET, None).await?;

    Ok(data)
}

pub async fn get_follower_timeline(
    xplore: &mut Xplore,
    user_id: &str,
    max_items: i32,
    cursor: Option<String>,
) -> Result<RelationshipTimeline> {
    let max_items = if max_items > 50 { 50 } else { max_items };

    let mut variables = json!({
        "userId": user_id,
        "count": max_items,
        "includePromotedContent": false,
    });

    let features = json!({
        "responsive_web_twitter_article_tweet_consumption_enabled": false,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
        "longform_notetweets_inline_media_enabled": true,
        "responsive_web_media_download_video_enabled": false,

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
        "unified_cards_ad_metadata_container_dynamic_card_content_query_enabled": false
    });

    if let Some(cursor_val) = cursor {
        if !cursor_val.is_empty() {
            variables["cursor"] = json!(cursor_val);
        }
    }

    let url = format!(
        "https://x.com/i/api/graphql/rRXFSG5vR6drKr5M37YOTw/Followers?variables={}&features={}",
        urlencoding::encode(&variables.to_string()),
        urlencoding::encode(&features.to_string())
    );

    let (data, _) = api::send_request::<RelationshipTimeline>(&mut xplore.auth, &url, Method::GET, None).await?;

    Ok(data)
}
