pub mod request;
pub mod socket;
pub mod subscriptions;
pub mod types;

use anyhow::Context;
use request::{XrplRequest, XrplSubscription};
use serde_json::Value;
use socket::XrplSocket;
use tokio::sync::broadcast;

pub struct XrplClient {
    pub url: String,
    socket: XrplSocket,
}

impl XrplClient {
    pub async fn new(url: &str) -> anyhow::Result<XrplClient> {
        Ok(XrplClient {
            url: url.into(),
            socket: XrplSocket::new(url, None)
                .await
                .context("failed to initialise websocket")?,
        })
    }

    pub async fn call(&self, request: impl Into<Value>) -> anyhow::Result<String> {
        self.socket.request(request.into()).await
    }

    pub async fn request<T: XrplRequest>(&self, request: T) -> anyhow::Result<T::Response> {
        let response = self.call(request).await?;
        let parsed = serde_json::from_str::<T::Response>(&response)?;
        Ok(parsed)
    }

    pub async fn subscribe<T: XrplSubscription>(
        &self,
        request: T,
    ) -> anyhow::Result<(T::Response, broadcast::Receiver<T::Message>)> {
        let response = self.request(request).await?;
        let receiver = self.socket.subscribe::<T>().await;
        Ok((response, receiver))
    }
}
