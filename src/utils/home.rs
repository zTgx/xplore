use {
    crate::core::{
        client::Xplore,
        models::{
            home::{HomeTimelineResponse, TimelineInstruction},
            rel::RelationshipTimeline,
            Result,
        },
    },
    reqwest::Method,
    serde_json::{json, Value},
};

pub async fn fetch_home_timeline(xplore: &Xplore, count: i32, seen_tweet_ids: Vec<String>) -> Result<Vec<Value>> {
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

    let (response, _) = xplore.inner.rpc.send_request::<HomeTimelineResponse>(&url, Method::GET, None).await?;

    let home = response.data.map(|data| data.home.home_timeline.instructions);

    let mut entries = Vec::new();

    if let Some(instructions) = home {
        for instruction in instructions {
            match instruction {
                TimelineInstruction::AddEntries { entries: new_entries } => {
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
    xplore: &Xplore,
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

    let (data, _) = xplore.inner.rpc.send_request::<RelationshipTimeline>(&url, Method::GET, None).await?;

    Ok(data)
}

pub async fn get_follower_timeline(
    xplore: &Xplore,
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

    let (data, _) = xplore.inner.rpc.send_request::<RelationshipTimeline>(&url, Method::GET, None).await?;

    Ok(data)
}
