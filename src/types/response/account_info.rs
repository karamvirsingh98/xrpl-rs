use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountInfoResult {
    pub account_data: AccountData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountData {
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
