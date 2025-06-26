use {
    crate::{
        api::profile::IProfile,
        core::{
            client::Xplore,
            error::XploreError,
            models::{constants::URL_USER_BY_SCREEN_NAME, profile::*, Result},
        },
        utils::profile as profile_utils,
    },
    async_trait::async_trait,
    reqwest::Method,
};

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
