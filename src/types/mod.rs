pub mod account_object;
pub mod transaction;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Amount {
    Xrpl(String),
    IssuedCurrency {
        value: String,
        currency: String,
        issuer: String,
    },
}

impl Default for Amount {
    fn default() -> Self {
        Amount::Xrpl("0".into())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathStep {
    pub account: Option<String>,
    pub currency: Option<String>,
    pub isssuer: Option<String>,
}
