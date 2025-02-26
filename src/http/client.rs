use crate::error::{Result, TwitterError};
use crate::Xplore;
use reqwest::Method;
use serde::de::DeserializeOwned;

impl Xplore {
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.request(Method::GET, endpoint, None).await
    }

    pub async fn get_with_params<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> Result<T> {
        self.request(Method::GET, endpoint, params).await
    }

    pub async fn post<T: DeserializeOwned>(&self, endpoint: &str, params: Option<serde_json::Value>) -> Result<T> {
        self.request(Method::POST, endpoint, params).await
    }

    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> Result<T> {
        let mut headers = reqwest::header::HeaderMap::new();
        self.auth.install_headers(&mut headers).await?;

        let mut request = self.client.request(method, endpoint);
        request = request.headers(headers);

        if let Some(params) = params {
            request = request.json(&params);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(TwitterError::Api(format!("Request failed with status: {}", response.status())))
        }
    }
}
