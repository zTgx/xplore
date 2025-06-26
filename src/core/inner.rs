use crate::core::{models::Result, rpc::InnerRpc};

pub struct Inner {
    pub rpc: InnerRpc,
}

impl Inner {
    pub async fn new(cookie: &str) -> Result<Self> {
        let rpc = InnerRpc::new(cookie).await?;

        Ok(Inner { rpc })
    }
}
