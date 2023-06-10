use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use super::{XrplRequest, XrplResponse};

#[derive(Default, Serialize)]
pub struct ServerInfoRequest;

impl Into<Value> for ServerInfoRequest {
    fn into(self) -> Value {
        json!({
            "command": "server_info",
            "id": Uuid::new_v4().to_string()
        })
    }
}

impl XrplRequest for ServerInfoRequest {
    type Response = XrplResponse<ServerInfoResult>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoResult {
    pub info: ServerInfo,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfo {
    pub build_version: String,
    pub complete_ledgers: String,
    pub hostid: String,
    pub initial_sync_duration_us: String,
    pub io_latency_ms: i64,
    pub jq_trans_overflow: String,
    pub last_close: ServerInfoLastClose,
    pub load_factor: i64,
    pub network_id: i64,
    pub peer_disconnects: String,
    pub peer_disconnects_resources: String,
    pub peers: i64,
    pub pubkey_node: String,
    pub server_state: String,
    pub server_state_duration_us: String,
    pub state_accounting: ServerInfoStateAccounting,
    pub time: String,
    pub uptime: i64,
    pub validated_ledger: ServerInfoValidatedLedger,
    pub validation_quorum: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoValidatedLedger {
    pub age: i64,
    pub base_fee_xrp: f64,
    pub hash: String,
    pub reserve_base_xrp: i64,
    pub reserve_inc_xrp: i64,
    pub seq: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoLastClose {
    pub converge_time_s: f64,
    pub proposers: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoStateAccounting {
    pub connected: ServerInfoStateAccount,
    pub disconnected: ServerInfoStateAccount,
    pub full: ServerInfoStateAccount,
    pub syncing: ServerInfoStateAccount,
    pub tracking: ServerInfoStateAccount,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoStateAccount {
    pub duration_us: String,
    pub transitions: String,
}
