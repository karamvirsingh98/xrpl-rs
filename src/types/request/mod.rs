use serde_json::json;

#[derive(Debug)]
pub enum XrplRequest {
    ServerInfoRequest,
    // AccountChannelsRequest { account: String },
    // AccountCurrenciesRequest {
    //     account: String,
    // },
    AccountInfoRequest { account: String },
    // AccountLinesRequest {
    //     account: String,
    // },
    // AccountNFTsRequest {
    //     account: String,
    // },
    // AccountObjectsRequest {
    //     account: String,
    // },
    // AccountOffersRequest {
    //     account: String,
    // },
    // AccountTxRequest {
    //     account: String,
    // },
    // BookOffersRequest {
    //     taker_gets: String,
    //     taker_pays: String,
    // },
}

impl XrplRequest {
    pub fn to_string(&self, id: &str) -> String {
        match self {
            XrplRequest::ServerInfoRequest => {
                json!({ "id": id, "command": "server_info" }).to_string()
            }
            // XrplRequest::AccountChannelsRequest { account } => {
            //     json!({ "id": id, "command": "account_channels", "account": account }).to_string()
            // }
            // XrplRequest::AccountCurrenciesRequest { account } => {
            //     json!({ "id": id, "command": "account_currencies", "account": account }).to_string()
            // }
            XrplRequest::AccountInfoRequest { account } => {
                json!({ "id": id, "command": "account_info", "account": account }).to_string()
            }
            // XrplRequest::AccountLinesRequest { account } => {
            //     json!({ "id": id, "command": "account_info", "account": account }).to_string()
            // }
            // XrplRequest::AccountNFTsRequest { account } => {
            //     json!({ "id": id, "command": "account_info", "account": account }).to_string()
            // }
            // XrplRequest::AccountObjectsRequest { account } => {
            //     json!({ "id": id, "command": "account_info", "account": account }).to_string()
            // }
            // XrplRequest::AccountOffersRequest { account } => {
            //     json!({ "id": id, "command": "account_offers", "account": account }).to_string()
            // }
            // XrplRequest::AccountTxRequest { account } => {
            //     json!({ "id": id, "command": "account_tx", "account": account }).to_string()
            // }
            // XrplRequest::BookOffersRequest {
            //     taker_gets,
            //     taker_pays,
            // } => json!({ "id": id, "command": "book_offers", "taker_gets": taker_gets, "taker_pays": taker_pays }).to_string(),
        }
    }
}

#[derive(Clone, Debug)]

pub enum XrplSubscriptionRequest {
    LedgerSubscriptionRequest,
}

impl XrplSubscriptionRequest {
    pub fn to_string(&self) -> String {
        match self {
            XrplSubscriptionRequest::LedgerSubscriptionRequest => {
                json!({ "command": "subscribe", "streams": ["ledger"] }).to_string()
            }
        }
    }
}
