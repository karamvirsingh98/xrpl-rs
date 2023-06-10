use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::request::XrplRequest;

use super::XrplResponse;

#[derive(Default, Serialize)]
#[skip_serializing_none]
pub struct AccountLinesRequest {
    pub account: String,
    pub ledger_hash: Option<String>,
    pub peer: Option<String>,
    pub limit: Option<i64>,
    pub marker: Option<Value>,
}

impl Into<Value> for AccountLinesRequest {
    fn into(self) -> Value {
        let value = serde_json::to_value(self);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_lines".into());
        value.into()
    }
}

impl XrplRequest for AccountLinesRequest {
    type Response = XrplResponse<AccountLinesResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountLinesResult {
    pub account: String,
    pub lines: Vec<Trustline>,
    pub ledger_current_index: Option<i64>,
    pub ledger_index: Option<i64>,
    pub ledger_hash: Option<String>,
    pub marker: Option<Value>,
}

#[derive(Debug, Deserialize)]

pub struct Trustline {
    pub account: String,
    pub balance: String,
    pub currency: String,
    pub limit: String,
    pub limit_peer: String,
    pub quality_in: i64,
    pub quality_out: i64,
    pub no_ripple: Option<bool>,
    pub no_ripple_peer: Option<bool>,
    pub authorized: Option<bool>,
    pub peer_authorized: Option<bool>,
    pub freeze: Option<bool>,
    pub freeze_peer: Option<bool>,
}
