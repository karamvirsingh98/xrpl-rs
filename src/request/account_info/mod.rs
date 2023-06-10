use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::request::XrplRequest;

use super::XrplResponse;

#[derive(Default, Serialize)]
#[skip_serializing_none]
pub struct AccountInfoRequest {
    pub account: String,
    pub ledger_hash: Option<String>,
    pub queue: Option<bool>,
    pub signer_lists: Option<bool>,
    pub strict: Option<bool>,
}

impl Into<Value> for AccountInfoRequest {
    fn into(self) -> Value {
        let value = serde_json::to_value(self);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_info".into());
        value.into()
    }
}

impl XrplRequest for AccountInfoRequest {
    type Response = XrplResponse<AccountInfoResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountInfoResult {
    pub account_data: AccountRoot,
    pub signer_lists: Option<Vec<String>>,
    pub ledger_current_index: Option<i64>,
    pub ledger_index: Option<i64>,
    pub queue_data: Option<String>,
    pub validated: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountRoot {
    pub account: String,
    pub balance: String,
    pub flags: i64,
    pub ledger_entry_type: String,
    pub owner_count: i64,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    pub previous_txn_lgr_seq: i64,
    pub sequence: i64,
    #[serde(rename = "index")]
    pub index: String,
}

#[derive(Debug, Deserialize)]
pub struct QueueData {
    pub txn_count: i64,
    pub auth_change_queued: Option<bool>,
    pub lowest_sequence: Option<i64>,
    pub highest_sequence: Option<i64>,
    pub max_spend_drops_total: Option<String>,
    pub transactions: Option<Vec<QueueTransaction>>,
}

#[derive(Debug, Deserialize)]
pub struct QueueTransaction {
    pub auth_change: bool,
    pub fee: String,
    pub fee_level: String,
    pub max_spend_drops: String,
    pub seq: i64,
}
