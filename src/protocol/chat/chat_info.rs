/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::chatroom::ChatroomInfo;
use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(ChatInfoRequest, ChatInfoResponse)]
pub struct ChatInfo;

/// Request Chatroom info
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct ChatInfoRequest {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct ChatInfoResponse {
    /// Chatroom info
    #[serde(rename = "chatInfo")]
    pub chat_info: ChatroomInfo,

    /// Unknown. Only appears on openchat rooms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o: Option<i32>,
}
