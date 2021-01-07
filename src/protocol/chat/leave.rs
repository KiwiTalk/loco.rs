/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(LeaveRequest, LeaveResponse)]
pub struct Leave;

/// Leave chatroom
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct LeaveRequest {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Block chatroom. Cannot rejoin chatroom if true.
    pub block: bool,
}

/// Leave chatroom
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct LeaveResponse {
    /// Last token(?) id
    #[serde(rename = "lastTokenId")]
    pub last_token_id: i64,
}
