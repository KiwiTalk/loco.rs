/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(DeleteMsgRequest, DeleteMsgResponse)]
pub struct DeleteMsg;

/// Delete chat. Official server only deletes message sent before 5 mins max.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct DeleteMsgRequest {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chat log id
    #[serde(rename = "logId")]
    pub log_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct DeleteMsgResponse;
