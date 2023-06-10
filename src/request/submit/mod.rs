use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{request::XrplRequest, types::transaction::Transaction};

use super::XrplResponse;

#[derive(Default, Serialize)]
pub struct SubmitRequest {
    pub tx_blob: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_hard: Option<bool>,
}

impl Into<Value> for SubmitRequest {
    fn into(self) -> Value {
        let value = serde_json::to_value(self);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "submit".into());
        value.into()
    }
}

impl XrplRequest for SubmitRequest {
    type Response = XrplResponse<SubmitResponse>;
}

#[derive(Debug, Deserialize)]
pub struct SubmitResponse {
    pub engine_result: String,
    pub engine_result_code: i64,
    pub engine_result_message: String,
    pub tx_blob: String,
    pub tx_json: Transaction,
    pub accepted: bool,
    pub account_sequence_available: i64,
    pub account_sequence_next: i64,
    pub applied: bool,
    pub broadcast: bool,
    pub kept: bool,
    pub queued: bool,
    pub open_ledger_cost: String,
    pub validated_ledger_index: i64,
}
