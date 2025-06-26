use crate::core::{cookie::CookieTracker, inner::Inner, models::Result};

pub struct Xplore {
    pub inner: Inner,
    pub cookie_tracker: CookieTracker,
}

impl Xplore {
    pub async fn new(cookie: &str) -> Result<Self> {
        let inner = Inner::new(cookie).await?;
        let cookie_tracker = CookieTracker::new(cookie);

        Ok(Self { inner, cookie_tracker })
    }
}
