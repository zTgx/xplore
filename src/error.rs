use {
    thiserror::Error,
    serde::Deserialize,
};

#[derive(Debug, Error, Deserialize)]
pub enum XploreError {
    #[error("API error: {0}")]
    Api(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Network error: {0}")]
    #[serde(skip)]
    Network(#[from] reqwest::Error),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Missing environment variable: {0}")]
    EnvVar(String),

    #[error("Cookie error: {0}")]
    Cookie(String),

    #[error("JSON error: {0}")]
    #[serde(skip)]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    #[serde(skip)]
    Io(#[from] std::io::Error),
}