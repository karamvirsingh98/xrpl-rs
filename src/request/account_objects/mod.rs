use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{request::XrplRequest, types::account_object::AccountObject};

use super::XrplResponse;

#[derive(Default, Serialize)]
#[skip_serializing_none]
pub struct AccountObjectsRequest {
    pub account: String,
    #[serde(rename = "type")]
    pub kind: Option<AccountObjectRequestType>,
    pub deletion_blockers_only: Option<bool>,
    pub ledger_hash: Option<i64>,
    pub limit: Option<i64>,
    pub marker: Option<Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountObjectRequestType {
    Check,
    DepositPreauth,
    Escrow,
    NftOffer,
    Offer,
    PaymentChannel,
    SignerList,
    State,
    Ticket,
}

impl Into<Value> for AccountObjectsRequest {
    fn into(self) -> Value {
        let value = serde_json::to_value(self);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_objects".into());
        value.into()
    }
}

impl XrplRequest for AccountObjectsRequest {
    type Response = XrplResponse<AccountObjectsResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountObjectsResult {
    pub account: String,
    pub account_objects: Vec<AccountObject>,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<i64>,
    pub ledger_current_index: i64,
    pub limit: Option<i64>,
    pub marker: Option<Value>,
    pub validated: bool,
}
