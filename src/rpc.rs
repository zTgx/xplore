use crate::{
    auth::UserAuth,
    {Result, XploreError},
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart::Form,
    Client, Method,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

pub(crate) struct InnerRpc {
    pub client: Client,
    pub(crate) auth: UserAuth,
}

impl InnerRpc {
    pub async fn new(cookie: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .map_err(XploreError::Network)?;

        let mut auth = UserAuth::new().await?;
        auth.set_from_cookie_string(cookie).await?;

        Ok(Self { client, auth })
    }
}

impl InnerRpc {
    pub async fn send_request<T>(&self, url: &str, method: Method, body: Option<Value>) -> Result<(T, HeaderMap)>
    where
        T: DeserializeOwned,
    {
        let mut headers = HeaderMap::new();
        self.auth.install_headers(&mut headers).await?;

        let mut request = self.client.request(method, url).headers(headers);

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

    pub async fn request_multipart<T>(&self, url: &str, form: Form) -> Result<(T, HeaderMap)>
    where
        T: DeserializeOwned,
    {
        let mut headers = HeaderMap::new();
        self.auth.install_headers(&mut headers).await?;

        let request = self.client.request(Method::POST, url).headers(headers).multipart(form);

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
        &self,
        url: &str,
        user_name: &str,
        form_data: Vec<(String, String)>,
    ) -> Result<(T, HeaderMap)>
    where
        T: DeserializeOwned,
    {
        let mut headers = HeaderMap::new();
        self.auth.install_headers(&mut headers).await?;

        headers.insert("Content-Type", HeaderValue::from_str("application/x-www-form-urlencoded").unwrap());
        headers.insert("Referer", format!("https://twitter.com/{}", user_name).parse().unwrap());
        headers.insert("X-Twitter-Active-User", "yes".parse().unwrap());
        headers.insert("X-Twitter-Auth-Type", "OAuth2Session".parse().unwrap());
        headers.insert("X-Twitter-Client-Language", "en".parse().unwrap());

        let request = self.client.request(Method::POST, url).headers(headers).form(&form_data);

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
}
