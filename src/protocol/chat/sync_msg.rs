/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::chat::Chatlog;
use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(SyncMsgRequest, SyncMsgResponse)]
pub struct SyncMsg;

/// Sync skipped chats.
/// Official client send this when last log id written is different with actual last log id.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SyncMsgRequest {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Current written last chat log id in client.
    #[serde(rename = "cur")]
    pub current: i64,

    /// Max number to receive once.
    /// The default is 300. But the server always seems to send up to 300 regardless of the number.
    #[serde(rename = "cnt")]
    pub count: i32,

    /// Last chat log id received by server.
    pub max: i64,
}

/// Responses chatlogs between "current" and "max". Chatlog list sliced to 300 or "max" value max.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SyncMsgResponse {
    /// true if no more chat left below.
    #[serde(rename = "isOK")]
    is_ok: bool,

    /// Chatlog list
    #[serde(rename = "chatLogs")]
    chat_logs: Vec<Chatlog>,

    /// Unknown
    #[serde(rename = "jsi")]
    jsi: i64,

    #[serde(rename = "lastTokenId")]
    last_token_id: i64,
}
