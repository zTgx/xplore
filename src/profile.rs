use crate::error::{Result, TwitterError};
use crate::http::requests::request_api;
use crate::primitives::constants::{URL_USER_BY_REST_ID, URL_USER_BY_SCREEN_NAME};
use crate::primitives::profile::*;
use crate::{IProfile, Xplore};
use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::Method;

#[async_trait]
impl IProfile for Xplore {
    async fn get_profile_by_screen_name(&self, screen_name: &str) -> Result<Profile> {
        let mut headers = HeaderMap::new();
        self.auth.install_headers(&mut headers).await?;

        let (response, _) = request_api::<UserRaw>(
            &self.client,
            URL_USER_BY_SCREEN_NAME,
            headers,
            Method::GET,
            profile_utils::screen_name_body(screen_name),
        )
        .await?;

        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(TwitterError::Api(errors[0].message.clone()));
            }
        }

        let user_raw_result = &response.data.user.result;
        let mut legacy = user_raw_result.legacy.clone();
        let rest_id = user_raw_result.rest_id.clone();
        let is_blue_verified = user_raw_result.is_blue_verified;
        legacy.user_id = rest_id;
        if legacy.screen_name.is_none() || legacy.screen_name.as_ref().unwrap().is_empty() {
            return Err(TwitterError::Api(format!("Either {} does not exist or is private.", screen_name)));
        }
        Ok((&legacy, is_blue_verified).into())
    }

    #[allow(dead_code)]
    async fn get_screen_name_by_user_id(&self, user_id: &str) -> Result<String> {
        let mut headers = HeaderMap::new();
        self.auth.install_headers(&mut headers).await?;

        let (response, _) = request_api::<UserRaw>(
            &self.client,
            URL_USER_BY_REST_ID,
            headers,
            Method::GET,
            profile_utils::user_id_body(user_id),
        )
        .await?;

        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(TwitterError::Api(errors[0].message.clone()));
            }
        }

        if let Some(user) = response.data.user.result.legacy.screen_name {
            Ok(user)
        } else {
            Err(TwitterError::Api(format!("Either user with ID {} does not exist or is private.", user_id)))
        }
    }

    async fn get_user_id_by_screen_name(&self, screen_name: &str) -> Result<String> {
        let cache = ID_CACHE.clone();

        let mut cache = cache.lock().await;
        if let Some(cached_id) = cache.get(screen_name) {
            return Ok(cached_id.clone());
        }

        let profile = self.get_profile_by_screen_name(screen_name).await?;

        let user_id = profile.id;

        cache.insert(screen_name.to_string(), user_id.clone());

        Ok(user_id)
    }
}

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
