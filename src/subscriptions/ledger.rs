use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::request::{XrplRequest, XrplResponse, XrplSubscription};

#[derive(Serialize)]
pub struct LedgerClosedSubscription;

impl Into<Value> for LedgerClosedSubscription {
    fn into(self) -> Value {
        json!({
            "id": Uuid::new_v4().to_string(),
            "command": "subscribe",
            "streams": ["ledger"]
        })
    }
}

impl XrplRequest for LedgerClosedSubscription {
    type Response = XrplResponse<LedgerSubscriptionResponse>;
}

#[derive(Debug, Deserialize)]
pub struct LedgerSubscriptionResponse {
    pub fee_base: i64,
    pub fee_ref: i64,
    pub ledger_hash: String,
    pub ledger_index: i64,
    pub ledger_time: i64,
    pub reserve_base: i64,
    pub reserve_inc: i64,
    pub validated_ledgers: Option<String>,
}

impl XrplSubscription for LedgerClosedSubscription {
    type Message = LedgerSubscriptionMessage;
}

#[derive(Debug, Clone, Deserialize)]
pub struct LedgerSubscriptionMessage {
    pub fee_base: i64,
    pub fee_ref: i64,
    pub ledger_hash: String,
    pub ledger_index: i64,
    pub ledger_time: i64,
    pub reserve_base: i64,
    pub reserve_inc: i64,
    pub txn_count: i64,
    #[serde(rename = "type")]
    pub kind: String,
    pub validated_ledgers: Option<String>,
}
