use serde_derive::{Deserialize, Serialize};

use super::{Amount, PathStep};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "TransactionType")]
pub enum Transaction {
    #[serde(rename_all = "PascalCase")]
    NFTokenAcceptOffer {
        #[serde(rename = "NFTokenSellOffer")]
        nftoken_sell_offer: Option<String>,
        #[serde(rename = "NFTokenBuyOffer")]
        nftoken_buy_offer: Option<String>,
        #[serde(rename = "NFTokenBrokerFee")]
        nftoken_broker_fee: Option<Amount>,
        // flags: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    NFTokenBurn {
        account: String,
        #[serde(rename = "NFTokenID")]
        nftoken_id: String,
        owner: String,
    },
    #[serde(rename_all = "PascalCase")]
    NFTokenCancelOffer {
        #[serde(rename = "NFTokenOffers")]
        nftoken_offers: Vec<String>,
    },
    #[serde(rename_all = "PascalCase")]
    NFTokenCreateOffer {
        #[serde(rename = "NFTokenID")]
        nftoken_id: String,
        amount: Amount,
        owner: Option<String>,
        expiration: Option<i64>,
        destination: Option<String>,
        // flags: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    NFTokenMint {
        #[serde(rename = "NFTokenTaxon")]
        nftoken_taxon: String,
        issuer: String,
        transfer_fee: Option<i64>,
        uri: Option<String>,
        // flags: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    AccountSet {
        clear_flag: Option<i64>,
        domain: Option<String>,
        email_hash: Option<String>,
        message_key: Option<String>,
        set_flag: Option<i64>,
        transfer_rate: Option<i64>,
        tick_size: Option<i64>,
        #[serde(rename = "NFTokenMinter")]
        nftoken_minter: Option<i64>,
        // flags: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    TrustSet {
        limit_amount: Amount,
        quality_in: Option<i64>,
        quality_out: Option<i64>,
        // flags: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    OfferCreate {
        expiration: Option<i64>,
        offer_sequence: Option<i64>,
        taker_gets: Amount,
        taker_pays: Amount,
        // flags: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    Payment {
        account: String,
        amount: Amount,
        destination: String,
        destination_tag: Option<i64>,
        invoice_id: Option<String>,
        paths: Option<Vec<Vec<PathStep>>>,
        send_max: Option<Amount>,
        deliver_min: Option<Amount>,
        // flags: Option<i64>,
    },
}

impl Into<String> for Transaction {
    fn into(self) -> String {
        serde_json::to_value(self).unwrap().to_string()
    }
}
