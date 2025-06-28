use {
    crate::{Result, XploreError},
    async_trait::async_trait,
    log::info,
    reqwest::Response,
    std::time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Information about a rate-limiting event. Both the request and response
/// information are provided.
pub struct RateLimitEvent {
    /// The complete arguments that were passed to the fetch function.
    pub fetch_parameters: String,
    /// The failing HTTP response.
    pub response: Response,
}

/// The public interface for all rate-limiting strategies. Library consumers are
/// welcome to provide their own implementations of this trait in the Scraper
/// constructor options.
///
/// The `RateLimitEvent` object contains both the request and response
/// information associated with the event.
#[async_trait]
pub trait RateLimitStrategy {
    /// Called when the scraper is rate limited.
    ///
    /// # Arguments
    /// * `event` - The event information, including the request and response info.
    async fn on_rate_limit(&self, event: RateLimitEvent) -> Result<()>;
}

/// A rate-limiting strategy that simply waits for the current rate limit period to expire.
/// This has been known to take up to 13 minutes, in some cases.
pub struct WaitingRateLimitStrategy;

#[async_trait]
impl RateLimitStrategy for WaitingRateLimitStrategy {
    async fn on_rate_limit(&self, event: RateLimitEvent) -> Result<()> {
        /*
          Known headers at this point:
          - x-rate-limit-limit: Maximum number of requests per time period?
          - x-rate-limit-reset: UNIX timestamp when the current rate limit will be reset.
          - x-rate-limit-remaining: Number of requests remaining in current time period?
        */
        let x_rate_limit_limit = match event.response.headers().get("x-rate-limit-limit") {
            Some(header) => header.to_str().map_err(|_e| XploreError::RateLimit)?.to_owned(),
            None => return Err(XploreError::InvalidResponse("Missing x-rate-limit-limit header".into())),
        };

        let x_rate_limit_remaining = match event.response.headers().get("x-rate-limit-remaining") {
            Some(header) => header.to_str().map_err(|_e| XploreError::RateLimit)?.to_owned(),
            None => return Err(XploreError::InvalidResponse("Missing x-rate-limit-remaining header".into())),
        };

        let x_rate_limit_reset = match event.response.headers().get("x-rate-limit-reset") {
            Some(header) => header.to_str().map_err(|_e| XploreError::RateLimit)?.to_owned(),
            None => return Err(XploreError::InvalidResponse("Missing x-rate-limit-reset header".into())),
        };

        info!(
            "Rate limit event: limit={}, remaining={}, reset={}",
            x_rate_limit_limit, x_rate_limit_remaining, x_rate_limit_reset
        );

        if x_rate_limit_remaining == "0" && !x_rate_limit_reset.is_empty() {
            let reset_time = x_rate_limit_reset
                .parse::<u64>()
                .map_err(|e| XploreError::InvalidResponse(format!("Failed to parse x-rate-limit-reset: {}", e)))?;
            let current_time =
                SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_e| XploreError::RateLimit)?.as_secs();
            let time_delta_ms = (reset_time - current_time) * 1000;

            // I have seen this block for 800s (~13 *minutes*)
            tokio::time::sleep(Duration::from_millis(time_delta_ms)).await;
        }

        Ok(())
    }
}

/// A rate-limiting strategy that throws an `ApiError` when a rate limiting event occurs.
pub struct ErrorRateLimitStrategy;

#[async_trait]
impl RateLimitStrategy for ErrorRateLimitStrategy {
    async fn on_rate_limit(&self, _event: RateLimitEvent) -> Result<()> {
        Err(XploreError::RateLimit)

        // let status = event.response.status();
        // let body = event.response.text().await.map_err(|e| XploreError::Network(e.into()))?;
        // Err(XploreError::Api(format!("Rate limit exceeded. Status: {}, Body: {}", status, body)))
    }
}
