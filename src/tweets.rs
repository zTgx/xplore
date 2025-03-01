use crate::{
    primitives::{endpoints::Endpoints, tweets::Tweet, TweetRetweetResponse},
    timeline::v2::{parse_threaded_conversation, parse_timeline_tweets_v2, QueryTweetsResponse, ThreadedConversation},
    IProfile, ITweet, Result, TwitterError, Xplore,
};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

#[async_trait]
impl ITweet for Xplore {
    async fn post_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        create_tweet_request(self, text, reply_to, media_data).await
    }

    async fn read_tweet(&self, tweet_id: &str) -> Result<Tweet> {
        get_tweet(self, tweet_id).await
    }

    async fn retweet(&self, tweet_id: &str) -> Result<TweetRetweetResponse> {
        let value = retweet(self, tweet_id).await?;
        let res = serde_json::from_value(value)?;

        Ok(res)
    }

    async fn like_tweet(&self, tweet_id: &str) -> Result<Value> {
        let value = like_tweet(self, tweet_id).await?;
        Ok(value)
    }

    async fn get_user_tweets(&self, user_id: &str, limit: usize) -> Result<Vec<Tweet>> {
        let url = format!("https://api.twitter.com/2/users/{}/tweets", user_id);
        let body = json!({
            "max_results": limit,
            "tweet.fields": "created_at,author_id,conversation_id,public_metrics"
        });

        let (v, _) = self.inner.rpc.send_request::<Vec<Tweet>>(&url, Method::GET, Some(body)).await?;
        Ok(v)
    }
}

pub async fn fetch_tweets(xplore: &Xplore, user_id: &str, max_tweets: i32, cursor: Option<&str>) -> Result<Value> {
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
    let (value, _) = xplore.inner.rpc.send_request(url, method, body).await?;

    Ok(value)
}

pub async fn fetch_tweets_and_replies(
    xplore: &Xplore,
    username: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<QueryTweetsResponse> {
    let user_id = xplore.get_user_id(username).await?;

    let endpoint = Endpoints::user_tweets_and_replies(&user_id, max_tweets.min(40), cursor);
    let url = &endpoint.to_request_url();
    let (value, _) = xplore.inner.rpc.send_request(url, Method::GET, None).await?;

    let parsed_response = parse_timeline_tweets_v2(&value);
    Ok(parsed_response)
}

pub async fn fetch_tweets_and_replies_by_user_id(
    xplore: &Xplore,
    user_id: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<QueryTweetsResponse> {
    let endpoint = Endpoints::user_tweets_and_replies(user_id, max_tweets.min(40), cursor);
    let url = &endpoint.to_request_url();
    let method = Method::GET;

    let (value, _headers) = xplore.inner.rpc.send_request(url, method, None).await?;

    let parsed_response = parse_timeline_tweets_v2(&value);
    Ok(parsed_response)
}

pub async fn fetch_list_tweets(xplore: &Xplore, list_id: &str, max_tweets: i32, cursor: Option<&str>) -> Result<Value> {
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

    let (value, _) = xplore.inner.rpc.send_request(url, Method::GET, body).await?;

    Ok(value)
}

pub async fn create_quote_tweet(
    xplore: &Xplore,
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
    let (v, _) = xplore.inner.rpc.send_request(url, Method::POST, body).await?;

    Ok(v)
}

pub async fn like_tweet(xplore: &Xplore, tweet_id: &str) -> Result<Value> {
    let url = "https://twitter.com/i/api/graphql/lI07N6Otwv1PhnEgXILM7A/FavoriteTweet";
    let body = Some(json!({
        "variables": {
            "tweet_id": tweet_id
        }
    }));

    let (value, _) = xplore.inner.rpc.send_request(url, Method::POST, body).await?;
    Ok(value)
}

pub async fn retweet(xplore: &Xplore, tweet_id: &str) -> Result<Value> {
    let url = "https://twitter.com/i/api/graphql/ojPdsZsimiJrUGLR1sjUtA/CreateRetweet";
    let body = Some(json!({
        "variables": {
            "tweet_id": tweet_id,
            "dark_request": false
        }
    }));
    let (value, _) = xplore.inner.rpc.send_request(url, Method::POST, body).await?;
    Ok(value)
}

pub async fn create_long_tweet(
    xplore: &Xplore,
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
    let (value, _) = xplore.inner.rpc.send_request(url, Method::POST, body).await?;

    Ok(value)
}

pub async fn fetch_liked_tweets(
    xplore: &Xplore,
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
    let (value, _) = xplore.inner.rpc.send_request(url, Method::POST, body).await?;
    Ok(value)
}

pub async fn upload_media(xplore: &Xplore, file_data: Vec<u8>, media_type: &str) -> Result<String> {
    let upload_url = "https://upload.twitter.com/1.1/media/upload.json";

    // Check if media is video
    let is_video = media_type.starts_with("video/");

    if is_video {
        // Handle video upload using chunked upload
        upload_video_in_chunks(xplore, file_data, media_type).await
    } else {
        // Handle image upload directly
        let form = reqwest::multipart::Form::new().part("media", reqwest::multipart::Part::bytes(file_data));

        let (response, _) = xplore.inner.rpc.request_multipart::<Value>(upload_url, form).await?;

        response["media_id_string"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| TwitterError::Api("Failed to get media_id".into()))
    }
}

async fn upload_video_in_chunks(xplore: &Xplore, file_data: Vec<u8>, media_type: &str) -> Result<String> {
    let upload_url = "https://upload.twitter.com/1.1/media/upload.json";

    let body = Some(json!({
        "command": "INIT",
        "total_bytes": file_data.len(),
        "media_type": media_type
    }));
    let (init_response, _) = xplore.inner.rpc.send_request::<Value>(&upload_url, Method::POST, body).await?;

    let media_id = init_response["media_id_string"]
        .as_str()
        .ok_or_else(|| TwitterError::Api("Failed to get media_id".into()))?
        .to_string();

    // APPEND command - upload in chunks
    let chunk_size = 5 * 1024 * 1024; // 5MB chunks
    let mut segment_index = 0;

    for chunk in file_data.chunks(chunk_size) {
        let form = reqwest::multipart::Form::new()
            .text("command", "APPEND")
            .text("media_id", media_id.clone())
            .text("segment_index", segment_index.to_string())
            .part("media", reqwest::multipart::Part::bytes(chunk.to_vec()));

        let _ = xplore.inner.rpc.request_multipart::<Value>(upload_url, form).await?;

        segment_index += 1;
    }

    // FINALIZE command
    let (finalize_response, _) = xplore
        .inner
        .rpc
        .send_request::<Value>(&format!("{}?command=FINALIZE&media_id={}", upload_url, media_id), Method::POST, None)
        .await?;

    // Check processing status for videos
    if finalize_response.get("processing_info").is_some() {
        check_upload_status(xplore, &media_id).await?;
    }

    Ok(media_id)
}

async fn check_upload_status(xplore: &Xplore, media_id: &str) -> Result<()> {
    let upload_url = "https://upload.twitter.com/1.1/media/upload.json";

    for _ in 0..20 {
        // Maximum 20 attempts
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Wait 5 seconds

        let url = &format!("{}?command=STATUS&media_id={}", upload_url, media_id);
        let method = Method::GET;
        let body = None;
        let (status_response, _) = xplore.inner.rpc.send_request::<Value>(url, method, body).await?;

        if let Some(processing_info) = status_response.get("processing_info") {
            match processing_info["state"].as_str() {
                Some("succeeded") => return Ok(()),
                Some("failed") => return Err(TwitterError::Api("Video processing failed".into())),
                _ => continue,
            }
        }
    }

    Err(TwitterError::Api("Video processing timeout".into()))
}

pub async fn get_tweet(xplore: &Xplore, id: &str) -> Result<Tweet> {
    let tweet_detail_request = Endpoints::tweet_detail(id);
    let url = tweet_detail_request.to_request_url();

    let (response, _) = xplore.inner.rpc.send_request::<Value>(&url, Method::GET, None).await?;
    let data = response.clone();
    let conversation: ThreadedConversation = serde_json::from_value(data)?;
    let tweets = parse_threaded_conversation(&conversation);
    tweets.into_iter().next().ok_or_else(|| TwitterError::Api("No tweets found".into()))
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
    xplore: &Xplore,
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
    let (value, _) = xplore.inner.rpc.send_request(url, Method::POST, body).await?;
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
    xplore: &Xplore,
    user_id: &str,
    max_tweets: i32,
    cursor: Option<&str>,
) -> Result<QueryTweetsResponse> {
    let endpoint = Endpoints::user_tweets(user_id, max_tweets.min(200), cursor);
    let url = &endpoint.to_request_url();

    let (value, _) = xplore.inner.rpc.send_request(url, Method::GET, None).await?;

    let parsed_response = parse_timeline_tweets_v2(&value);
    Ok(parsed_response)
}
