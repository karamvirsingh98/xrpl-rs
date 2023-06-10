use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{request::XrplRequest, types::Amount};

use super::XrplResponse;

#[derive(Default, Serialize)]
#[skip_serializing_none]
pub struct AccountOffersRequest {
    pub account: String,
    pub limit: Option<i64>,
    pub marker: Option<Value>,
}

impl Into<Value> for AccountOffersRequest {
    fn into(self) -> Value {
        let value = serde_json::to_value(self);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_offers".into());
        value.into()
    }
}

impl XrplRequest for AccountOffersRequest {
    type Response = XrplResponse<AccountOffersResponse>;
}

#[derive(Debug, Deserialize)]
pub struct AccountOffersResponse {
    pub account: String,
    pub account_nfts: Vec<AccountOffer>,
    pub ledger_current_index: Option<i64>,
    pub ledger_index: Option<i64>,
    pub ledger_hash: Option<String>,
    pub marker: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountOffer {
    pub flags: i64,
    pub seq: i64,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
    pub quality: String,
    pub expiration: Option<i64>,
}
