use serde::{Deserialize, Serialize};

use super::chat::{ChatLog, LogId};

pub type ChannelId = i64;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ChannelData {
    #[serde(rename = "c")]
    channel_id: ChannelId,
    #[serde(rename = "t")]
    channel_type: String,
    #[serde(rename = "a")]
    active_user: usize,
    n: i32,
    last_seen_log_id: LogId,
    #[serde(rename = "l")]
    last_chat_log: Option<ChatLog>,
    i: Option<Vec<i64>>,
    k: Option<Vec<String>>,
    // m: Option<unknown>,
    // mmr: unknown,
    #[serde(rename = "ll")]
    last_log_id: LogId,
    #[serde(rename = "o")]
    last_update: i32,
    jn: i32,
    p: bool,
    li: Option<i64>,
    otk: Option<i32>,
}
