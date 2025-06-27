use {
    crate::{core::auth::UserAuth, Result, XploreError},
    reqwest::{
        header::{HeaderMap, HeaderValue},
        multipart::Form,
        Method,
    },
    serde::de::DeserializeOwned,
    serde_json::Value,
};

// https://github.com/elizaOS/agent-twitter-client/blob/main/src/api.ts
pub const BEARER_TOKEN: &str =
    "AAAAAAAAAAAAAAAAAAAAAFQODgEAAAAAVHTp76lzh3rFzcHbmHVvQxYYpTw%3DckAlMINMjmCwxUcaXbAN4XqJVdgMJaHqNOFgPMK0zN1qLqLQCF";

pub const URL_USER_BY_REST_ID: &str = "https://twitter.com/i/api/graphql/xf3jd90KKBCUxdlI_tNHZw/UserByRestId";

pub async fn send_request<T>(
    auth: &mut UserAuth,
    url: &str,
    method: Method,
    body: Option<Value>,
) -> Result<(T, HeaderMap)>
where
    T: DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    auth.install_headers(&mut headers).await?;

    let mut request = auth.client.request(method, url).headers(headers);

    if let Some(json_body) = body {
        request = request.json(&json_body);
    }

    let response = request.send().await?;

    if response.status().is_success() {
        let headers = response.headers().clone();
        let text = response.text().await?;
        let parsed: T = serde_json::from_str(&text)?;

        Ok((parsed, headers))
    } else {
        Err(XploreError::Api(format!("Request failed with status: {}", response.status())))
    }
}

pub async fn request_multipart<T>(auth: &mut UserAuth, url: &str, form: Form) -> Result<(T, HeaderMap)>
where
    T: DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    auth.install_headers(&mut headers).await?;

    let request = auth.client.request(Method::POST, url).headers(headers).multipart(form);

    let response = request.send().await?;

    if response.status().is_success() {
        let headers = response.headers().clone();
        let text = response.text().await?;
        let parsed: T = serde_json::from_str(&text)?;
        Ok((parsed, headers))
    } else {
        Err(XploreError::Api(format!("Request failed with status: {}", response.status())))
    }
}

pub async fn request_form<T>(
    auth: &mut UserAuth,
    url: &str,
    user_name: &str,
    form_data: Vec<(String, String)>,
) -> Result<(T, HeaderMap)>
where
    T: DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    auth.install_headers(&mut headers).await?;

    headers.insert("Content-Type", HeaderValue::from_str("application/x-www-form-urlencoded").unwrap());
    headers.insert("Referer", format!("https://twitter.com/{}", user_name).parse().unwrap());
    headers.insert("X-Twitter-Active-User", "yes".parse().unwrap());
    headers.insert("X-Twitter-Auth-Type", "OAuth2Session".parse().unwrap());
    headers.insert("X-Twitter-Client-Language", "en".parse().unwrap());

    let request = auth.client.request(Method::POST, url).headers(headers).form(&form_data);

    let response = request.send().await?;

    if response.status().is_success() {
        let headers = response.headers().clone();
        let text = response.text().await?;
        let parsed: T = serde_json::from_str(&text)?;
        Ok((parsed, headers))
    } else {
        Err(XploreError::Api(format!("Request failed with status: {}", response.status())))
    }
}
