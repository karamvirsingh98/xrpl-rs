use serde_derive::{Deserialize, Serialize};

use self::account_info::AccountInfoResult;
use self::server_info::ServerInfoResult;

pub mod account_info;
pub mod server_info;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LedgerSubscriptionResponse {
    #[serde(rename = "type")]
    pub kind: String,
    pub fee_base: i64,
    pub fee_ref: i64,
    pub ledger_hash: String,
    pub ledger_index: i64,
    pub ledger_time: i64,
    pub reserve_base: i64,
    pub reserve_inc: i64,
    pub validated_ledgers: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct XrplResponse<T> {
    pub id: String,
    pub result: T,
    #[serde(rename = "type")]
    pub kind: String,
    pub status: String,
}

pub type AccountInfoResponse = XrplResponse<AccountInfoResult>;
pub type ServerInfoResponse = XrplResponse<ServerInfoResult>;
