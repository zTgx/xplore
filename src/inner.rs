use crate::{cookie::CookieTracker, error::Result, rpc::InnerRpc};

pub(crate) struct Inner {
    pub(crate) cookie_tracker: CookieTracker,
    pub(crate) rpc: InnerRpc,
}

impl Inner {
    pub async fn new(cookie: &str) -> Result<Self> {
        let cookie_tracker = CookieTracker::new(cookie);

        let rpc = InnerRpc::new(cookie).await?;

        Ok(Inner {
            cookie_tracker,
            rpc,
        })
    }
}