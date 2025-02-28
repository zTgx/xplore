use crate::error::Result;
use crate::timeline::v1::{QueryProfilesResponse, QueryTweetsResponse};
use crate::{ISearch, XYZ};
use async_trait::async_trait;

#[derive(Debug, Clone, Copy)]
pub enum SearchMode {
    Top,
    Latest,
    Photos,
    Videos,
    Users,
}

#[async_trait]
impl ISearch for XYZ {
    async fn search_tweets(
        &self,
        query: &str,
        max_tweets: i32,
        search_mode: SearchMode,
        cursor: Option<String>,
    ) -> Result<QueryTweetsResponse> {
        let timeline = search_utils::get_search_timeline(self, query, max_tweets, search_mode, cursor).await?;

        Ok(search_utils::parse_search_timeline_tweets(&timeline))
    }

    async fn search_profiles(
        &self,
        query: &str,
        max_profiles: i32,
        cursor: Option<String>,
    ) -> Result<QueryProfilesResponse> {
        let timeline = search_utils::get_search_timeline(self, query, max_profiles, SearchMode::Users, cursor).await?;

        Ok(search_utils::parse_search_timeline_users(&timeline))
    }
}

mod search_utils {

    use super::SearchMode;
    use crate::error::Result;
    use crate::primitives::Profile;
    use crate::timeline::v1::{QueryProfilesResponse, QueryTweetsResponse};
    use crate::timeline::v2::{parse_legacy_tweet, SearchEntryRaw};
    use crate::XYZ;
    use lazy_static::lazy_static;
    use reqwest::Method;
    use serde::Deserialize;
    use serde_json::json;

    lazy_static! {
        static ref EMPTY_INSTRUCTIONS: Vec<SearchInstruction> = Vec::new();
        static ref EMPTY_ENTRIES: Vec<SearchEntryRaw> = Vec::new();
    }

    #[derive(Debug, Deserialize)]
    pub struct SearchTimeline {
        pub data: Option<SearchData>,
    }

    #[derive(Debug, Deserialize)]
    pub struct SearchData {
        pub search_by_raw_query: Option<SearchByRawQuery>,
    }

    #[derive(Debug, Deserialize)]
    pub struct SearchByRawQuery {
        pub search_timeline: Option<SearchTimelineData>,
    }

    #[derive(Debug, Deserialize)]
    pub struct SearchTimelineData {
        pub timeline: Option<TimelineData>,
    }

    #[derive(Debug, Deserialize)]
    pub struct TimelineData {
        pub instructions: Option<Vec<SearchInstruction>>,
    }

    #[derive(Debug, Deserialize)]
    pub struct SearchInstruction {
        pub entries: Option<Vec<SearchEntryRaw>>,
        pub entry: Option<SearchEntryRaw>,
        #[serde(rename = "type")]
        pub instruction_type: Option<String>,
    }

    pub(crate) async fn get_search_timeline(
        xyz: &XYZ,
        query: &str,
        max_items: i32,
        search_mode: SearchMode,
        _cursor: Option<String>,
    ) -> Result<SearchTimeline> {
        let max_items = if max_items > 50 { 50 } else { max_items };

        let mut variables = json!({
            "rawQuery": query,
            "count": max_items,
            "querySource": "typed_query",
            "product": "Top"
        });

        // Set product based on search mode
        match search_mode {
            SearchMode::Latest => {
                variables["product"] = json!("Latest");
            }
            SearchMode::Photos => {
                variables["product"] = json!("Photos");
            }
            SearchMode::Videos => {
                variables["product"] = json!("Videos");
            }
            SearchMode::Users => {
                variables["product"] = json!("People");
            }
            _ => {}
        }

        let features = json!({
            "longform_notetweets_inline_media_enabled": true,
            "responsive_web_enhance_cards_enabled": false,
            "responsive_web_media_download_video_enabled": false,
            "responsive_web_twitter_article_tweet_consumption_enabled": false,
            "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
            "interactive_text_enabled": false,
            "responsive_web_text_conversations_enabled": false,
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
            "unified_cards_ad_metadata_container_dynamic_card_content_query_enabled": false
        });

        let field_toggles = json!({
            "withArticleRichContentState": false
        });

        let params = [
            ("variables", serde_json::to_string(&variables)?),
            ("features", serde_json::to_string(&features)?),
            ("fieldToggles", serde_json::to_string(&field_toggles)?),
        ];

        let query_string =
            params.iter().map(|(k, v)| format!("{}={}", k, urlencoding::encode(v))).collect::<Vec<_>>().join("&");

        let url = format!("https://api.twitter.com/graphql/gkjsKepM6gl_HmFWoWKfgg/SearchTimeline?{}", query_string);

        let (res, _) = xyz.inner.rpc.send_request::<SearchTimeline>(&url, Method::GET, None).await?;

        Ok(res)
    }

    pub fn parse_search_timeline_tweets(timeline: &SearchTimeline) -> QueryTweetsResponse {
        let mut bottom_cursor = None;
        let mut top_cursor = None;
        let mut tweets = Vec::new();

        let instructions = timeline
            .data
            .as_ref()
            .and_then(|data| data.search_by_raw_query.as_ref())
            .and_then(|search| search.search_timeline.as_ref())
            .and_then(|timeline| timeline.timeline.as_ref())
            .and_then(|timeline| timeline.instructions.as_ref())
            .unwrap_or(&EMPTY_INSTRUCTIONS);

        for instruction in instructions {
            if let Some(instruction_type) = &instruction.instruction_type {
                if instruction_type == "TimelineAddEntries" || instruction_type == "TimelineReplaceEntry" {
                    if let Some(entry) = &instruction.entry {
                        if let Some(content) = &entry.content {
                            match content.cursor_type.as_deref() {
                                Some("Bottom") => {
                                    bottom_cursor = content.value.clone();
                                    continue;
                                }
                                Some("Top") => {
                                    top_cursor = content.value.clone();
                                    continue;
                                }
                                _ => {}
                            }
                        }
                    }

                    // Process entries
                    let entries = instruction.entries.as_ref().unwrap_or(&EMPTY_ENTRIES);
                    for entry in entries {
                        if let Some(content) = &entry.content {
                            if let Some(item_content) = &content.item_content {
                                if item_content.tweet_display_type.as_deref() == Some("Tweet") {
                                    if let Some(tweet_results) = &item_content.tweet_results {
                                        if let Some(result) = &tweet_results.result {
                                            let user_legacy = result
                                                .core
                                                .as_ref()
                                                .and_then(|core| core.user_results.as_ref())
                                                .and_then(|user_results| user_results.result.as_ref())
                                                .and_then(|result| result.legacy.as_ref());

                                            if let Ok(tweet_result) =
                                                parse_legacy_tweet(user_legacy, result.legacy.as_deref())
                                            {
                                                if tweet_result.views.is_none() {
                                                    if let Some(views) = &result.views {
                                                        if let Some(count) = &views.count {
                                                            if let Ok(view_count) = count.parse::<i32>() {
                                                                let mut tweet = tweet_result;
                                                                tweet.views = Some(view_count);
                                                                tweets.push(tweet);
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    tweets.push(tweet_result);
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if let Some(cursor_type) = &content.cursor_type {
                                match cursor_type.as_str() {
                                    "Bottom" => bottom_cursor = content.value.clone(),
                                    "Top" => top_cursor = content.value.clone(),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        QueryTweetsResponse { tweets, next: bottom_cursor, previous: top_cursor }
    }

    pub fn parse_search_timeline_users(timeline: &SearchTimeline) -> QueryProfilesResponse {
        let mut bottom_cursor = None;
        let mut top_cursor = None;
        let mut profiles = Vec::new();

        let instructions = timeline
            .data
            .as_ref()
            .and_then(|data| data.search_by_raw_query.as_ref())
            .and_then(|search| search.search_timeline.as_ref())
            .and_then(|timeline| timeline.timeline.as_ref())
            .and_then(|timeline| timeline.instructions.as_ref())
            .unwrap_or(&EMPTY_INSTRUCTIONS);

        for instruction in instructions {
            if let Some(instruction_type) = &instruction.instruction_type {
                if instruction_type == "TimelineAddEntries" || instruction_type == "TimelineReplaceEntry" {
                    if let Some(entry) = &instruction.entry {
                        if let Some(content) = &entry.content {
                            match content.cursor_type.as_deref() {
                                Some("Bottom") => {
                                    bottom_cursor = content.value.clone();
                                    continue;
                                }
                                Some("Top") => {
                                    top_cursor = content.value.clone();
                                    continue;
                                }
                                _ => {}
                            }
                        }
                    }

                    // Process entries
                    let entries = instruction.entries.as_ref().unwrap_or(&EMPTY_ENTRIES);
                    for entry in entries {
                        if let Some(content) = &entry.content {
                            if let Some(item_content) = &content.item_content {
                                if item_content.user_display_type.as_deref() == Some("User") {
                                    if let Some(user_results) = &item_content.user_results {
                                        if let Some(result) = &user_results.result {
                                            if let Some(legacy) = &result.legacy {
                                                let mut profile: Profile = (legacy, result.is_blue_verified).into();

                                                if profile.id.is_empty() {
                                                    profile.id = result.rest_id.clone().unwrap_or_default();
                                                }

                                                profiles.push(profile);
                                            }
                                        }
                                    }
                                }
                            } else if let Some(cursor_type) = &content.cursor_type {
                                match cursor_type.as_str() {
                                    "Bottom" => bottom_cursor = content.value.clone(),
                                    "Top" => top_cursor = content.value.clone(),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        QueryProfilesResponse { profiles, next: bottom_cursor, previous: top_cursor }
    }
}
