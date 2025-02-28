use crate::{error::Result, rpc::InnerRpc};

pub(crate) struct Inner {
    pub(crate) rpc: InnerRpc,
}

impl Inner {
    pub async fn new(cookie: &str) -> Result<Self> {
        let rpc = InnerRpc::new(cookie).await?;

        Ok(Inner { rpc })
    }
}
