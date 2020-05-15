use serde::{Deserialize, Serialize};

use crate::packet::LocoRequest;
use crate::types::chat_room::log;

#[derive(Serialize)]
pub struct SyncMessageRequest {
    #[serde(rename = "chatId")]
    pub channel_id: i64,
    #[serde(rename = "cur")]
    pub start_log_id: i64,
    #[serde(rename = "cnt")]
    pub count: i32,
    #[serde(rename = "max")]
    pub current_log_id: i64,
}

impl From<SyncMessageRequest> for LocoRequest {
    fn from(packet: SyncMessageRequest) -> Self {
        packet.into()
    }
}

#[derive(Deserialize)]
pub struct SyncMessageResponse {
    #[serde(rename = "isOK")]
    is_ok: bool,
    #[serde(rename = "chatLogs")]
    chat_list: Vec<log::ChatLog>,
    #[serde(rename = "jsi", default = "default_jsi")]
    jsi: i64,
    #[serde(rename = "lastTokenId", default = "default_last_token_id")]
    last_token_id: i64,
}

fn default_jsi() -> i64 {
    -1
}

fn default_last_token_id() -> i64 {
    0
}
