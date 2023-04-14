pub mod socket;
pub mod types;

use anyhow::Context;
use socket::XrplSocket;
use tokio::sync::broadcast::Receiver;
use types::{
    request::{XrplRequest, XrplSubscriptionRequest},
    response::{AccountInfoResponse, LedgerSubscriptionResponse, ServerInfoResponse},
};

pub struct XrplClient {
    pub url: String,
    socket: XrplSocket,
}

impl XrplClient {
    pub async fn new(url: &str) -> anyhow::Result<XrplClient> {
        let socket = XrplSocket::new(url)
            .await
            .context("failed to initialise websocket client")?;

        Ok(XrplClient {
            url: url.to_string(),
            socket,
        })
    }

    pub async fn get_server_info(&self) -> anyhow::Result<ServerInfoResponse> {
        self.socket
            .request(XrplRequest::ServerInfoRequest)
            .await
            .context("error getting server info")
    }

    pub async fn get_account_info(
        &self,
        account: impl Into<String>,
    ) -> anyhow::Result<AccountInfoResponse> {
        self.socket
            .request(XrplRequest::AccountInfoRequest {
                account: account.into(),
            })
            .await
            .context("error getting server info")
    }

    pub async fn subscribe_to_ledger(
        &self,
    ) -> anyhow::Result<Receiver<LedgerSubscriptionResponse>> {
        self.socket
            .subscribe(XrplSubscriptionRequest::LedgerSubscriptionRequest)
            .await
    }
}
