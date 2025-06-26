use serde_json::{json, Value};

fn get_screen_name_var(screen_name: &str) -> Value {
    json!({
        "screen_name": screen_name,
        "withSafetyModeUserFields": true
    })
}

fn get_screen_name_features() -> Value {
    json!({
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
    })
}

fn field_toggles(toggle: bool) -> Value {
    json!({
        "withAuxiliaryUserLabels": toggle
    })
}

pub fn screen_name_body(screen_name: &str) -> Option<Value> {
    Some(json!({
        "variables": get_screen_name_var(screen_name),
        "features": get_screen_name_features(),
        "fieldToggles": field_toggles(false)
    }))
}

/// user id
fn get_user_id_var(user_id: &str) -> Value {
    json!({
        "userId": user_id,
        "withSafetyModeUserFields": true
    })
}

fn get_user_id_features() -> Value {
    json!({
        "hidden_profile_subscriptions_enabled": true,
        "rweb_tipjar_consumption_enabled": true,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "verified_phone_label_enabled": false,
        "highlights_tweets_tab_ui_enabled": true,
        "responsive_web_twitter_article_notes_tab_enabled": true,
        "subscriptions_feature_can_gift_premium": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "responsive_web_graphql_timeline_navigation_enabled": true
    })
}

pub fn user_id_body(user_id: &str) -> Option<Value> {
    Some(json!({
        "variables": get_user_id_var(user_id),
        "features": get_user_id_features(),
    }))
}
