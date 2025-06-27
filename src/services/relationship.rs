use {
    crate::{
        core::models::{profile::Profile, Result},
        utils::relationship as rel_utils,
        Xplore,
    },
    serde_json::Value,
};

pub async fn get_following(
    xplore: &Xplore,
    user_id: &str,
    count: i32,
    cursor: Option<String>,
) -> Result<(Vec<Profile>, Option<String>)> {
    let response = rel_utils::fetch_profile_following(xplore, user_id, count, cursor).await?;
    Ok((response.profiles, response.next))
}

pub async fn get_followers(
    xplore: &Xplore,
    user_id: &str,
    count: i32,
    cursor: Option<String>,
) -> Result<(Vec<Profile>, Option<String>)> {
    let response = rel_utils::fetch_profile_followers(xplore, user_id, count, cursor).await?;
    Ok((response.profiles, response.next))
}

///! TODO: error handling
pub async fn follow(xplore: &Xplore, username: &str) -> Result<()> {
    let user_id = xplore.get_user_id(username).await?;

    let url = "https://api.twitter.com/1.1/friendships/create.json";

    let form = vec![
        ("include_profile_interstitial_type".to_string(), "1".to_string()),
        ("skip_status".to_string(), "true".to_string()),
        ("user_id".to_string(), user_id),
    ];

    let _ = xplore.inner.rpc.request_form::<Value>(url, username, form).await?;

    Ok(())
}

///! TODO: error handling
pub async fn unfollow(xplore: &Xplore, username: &str) -> Result<()> {
    let user_id = xplore.get_user_id(username).await?;

    let url = "https://api.twitter.com/1.1/friendships/destroy.json";

    let form = vec![
        ("include_profile_interstitial_type".to_string(), "1".to_string()),
        ("skip_status".to_string(), "true".to_string()),
        ("user_id".to_string(), user_id),
    ];

    let (_, _) = xplore.inner.rpc.request_form::<Value>(url, username, form).await?;

    Ok(())
}
