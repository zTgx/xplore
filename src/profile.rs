use crate::{
    primitives::{constants::URL_USER_BY_SCREEN_NAME, profile::*},
    IProfile, Result, error::XploreError, Xplore,
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl IProfile for Xplore {
    async fn get_profile(&self, screen_name: &str) -> Result<Profile> {
        let body = profile_utils::screen_name_body(screen_name);
        let response = self.inner.rpc.send_request::<UserRaw>(URL_USER_BY_SCREEN_NAME, Method::GET, body).await?;
        let user_raw = response.0;

        if let Some(errors) = user_raw.errors {
            if !errors.is_empty() {
                return Err(XploreError::Api(errors[0].message.clone()));
            }
        }

        let user_raw_result = &user_raw.data.user.result;
        let mut legacy = user_raw_result.legacy.clone();
        let rest_id = user_raw_result.rest_id.clone();
        let is_blue_verified = user_raw_result.is_blue_verified;
        legacy.user_id = rest_id;
        if legacy.screen_name.is_none() || legacy.screen_name.as_ref().unwrap().is_empty() {
            return Err(XploreError::Api(format!("Either {} does not exist or is private.", screen_name)));
        }
        Ok((&legacy, is_blue_verified).into())
    }

    async fn get_user_id(&self, screen_name: &str) -> Result<String> {
        let cache = ID_CACHE.clone();

        let mut cache = cache.lock().await;
        if let Some(cached_id) = cache.get(screen_name) {
            return Ok(cached_id.clone());
        }

        let profile = self.get_profile(screen_name).await?;

        let user_id = profile.id;

        cache.insert(screen_name.to_string(), user_id.clone());

        Ok(user_id)
    }
}

#[allow(dead_code)]
mod profile_utils {
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
}
