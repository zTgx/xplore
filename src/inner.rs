use crate::{rpc::InnerRpc, Result};

pub(crate) struct Inner {
    pub(crate) rpc: InnerRpc,
}

impl Inner {
    pub async fn new(cookie: &str) -> Result<Self> {
        let rpc = InnerRpc::new(cookie).await?;

        Ok(Inner { rpc })
    }
}
