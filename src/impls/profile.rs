use crate::api::requests::request_api;
use crate::error::{Result, TwitterError};
use crate::primitives::profile::*;
use crate::XploreX;
use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde_json::json;

impl XploreX {
    pub async fn get_profile_by_screen_name(&self, screen_name: &str) -> Result<Profile> {
        let mut headers = HeaderMap::new();
        self.auth.install_headers(&mut headers).await?;

        let variables = json!({
            "screen_name": screen_name,
            "withSafetyModeUserFields": true
        });

        let features = json!({
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
        });

        let field_toggles = json!({
            "withAuxiliaryUserLabels": false
        });

        let (response, _) = request_api::<UserRaw>(
            &self.client,
            "https://twitter.com/i/api/graphql/G3KGOASz96M-Qu0nwmGXNg/UserByScreenName",
            headers,
            Method::GET,
            Some(json!({
                "variables": variables,
                "features": features,
                "fieldToggles": field_toggles
            })),
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
            return Err(TwitterError::Api(format!(
                "Either {} does not exist or is private.",
                screen_name
            )));
        }
        Ok(parse_profile(&legacy, is_blue_verified))
    }

    #[allow(dead_code)]
    async fn get_screen_name_by_user_id(&self, user_id: &str) -> Result<String> {
        let mut headers = HeaderMap::new();
        self.auth.install_headers(&mut headers).await?;

        let variables = json!({
            "userId": user_id,
            "withSafetyModeUserFields": true
        });

        let features = json!({
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
        });

        let (response, _) = request_api::<UserRaw>(
            &self.client,
            "https://twitter.com/i/api/graphql/xf3jd90KKBCUxdlI_tNHZw/UserByRestId",
            headers,
            Method::GET,
            Some(json!({
                "variables": variables,
                "features": features
            })),
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
            Err(TwitterError::Api(format!(
                "Either user with ID {} does not exist or is private.",
                user_id
            )))
        }
    }

    pub async fn get_user_id_by_screen_name(&self, screen_name: &str) -> Result<String> {
        if let Some(cached_id) = ID_CACHE.lock().unwrap().get(screen_name) {
            return Ok(cached_id.clone());
        }

        let profile = self.get_profile_by_screen_name(screen_name).await?;
        if let Some(user_id) = Some(profile.id) {
            ID_CACHE
                .lock()
                .unwrap()
                .insert(screen_name.to_string(), user_id.clone());
            Ok(user_id)
        } else {
            Err(TwitterError::Api("User ID is undefined".into()))
        }
    }
}

pub fn parse_profile(user: &LegacyUserRaw, is_blue_verified: Option<bool>) -> Profile {
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
        is_blue_verified: Some(is_blue_verified.unwrap_or(false)),
        created_at: user
            .created_at
            .as_ref()
            .and_then(|date_str| {
                DateTime::parse_from_str(date_str, "%a %b %d %H:%M:%S %z %Y")
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .unwrap_or_else(Utc::now),
        profile_image_url: user
            .profile_image_url_https
            .as_ref()
            .map(|url| url.replace("_normal", "")),
        profile_banner_url: user.profile_banner_url.clone(),
        pinned_tweet_id: user
            .pinned_tweet_ids_str
            .as_ref()
            .and_then(|ids| ids.first().cloned()),
    };

    // Set website URL from entities using functional chaining
    user.entities
        .as_ref()
        .and_then(|entities| entities.url.as_ref())
        .and_then(|url_entity| url_entity.urls.as_ref())
        .and_then(|urls| urls.first())
        .and_then(|first_url| first_url.expanded_url.as_ref())
        .map(|expanded_url| profile.url = Some(expanded_url.clone()));

    profile
}
