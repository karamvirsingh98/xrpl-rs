use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::request::XrplRequest;

use super::XrplResponse;

#[derive(Default, Serialize)]
#[skip_serializing_none]
pub struct AccountCurrenciesRequest {
    pub account: String,
    pub ledger_hash: Option<String>,
    pub strict: Option<bool>,
}

impl Into<Value> for AccountCurrenciesRequest {
    fn into(self) -> Value {
        let value = serde_json::to_value(self);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_currencies".into());
        value.into()
    }
}

impl XrplRequest for AccountCurrenciesRequest {
    type Response = XrplResponse<AccountCurrenciesResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountCurrenciesResult {
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<i64>,
    pub receive_currencies: Vec<String>,
    pub send_currencies: Vec<String>,
    pub validated: bool,
}
