use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::request::XrplRequest;

use super::XrplResponse;

#[derive(Default, Serialize)]
#[skip_serializing_none]
pub struct AccountNftsRequest {
    pub account: String,
    pub limit: Option<i64>,
    pub marker: Option<Value>,
}

impl Into<Value> for AccountNftsRequest {
    fn into(self) -> Value {
        let value = serde_json::to_value(self);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_nfts".into());
        value.into()
    }
}

impl XrplRequest for AccountNftsRequest {
    type Response = XrplResponse<AccountNftsResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountNftsResult {
    pub account: String,
    pub account_nfts: Vec<AccountNFToken>,
    pub ledger_current_index: i64,
    pub validated: bool,
    pub marker: Option<Value>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountNFToken {
    pub flags: i64,
    pub issuer: String,
    #[serde(rename = "NFTokenID")]
    pub nftoken_id: String,
    #[serde(rename = "NFTokenTaxon")]
    pub nftoken_taxon: i64,
    pub uri: Option<String>,
    #[serde(rename = "NFTokenTaxon")]
    pub nft_serial: i64,
}
