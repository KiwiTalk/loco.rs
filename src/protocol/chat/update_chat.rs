/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(UpdateChatRequest, UpdateChatResponse)]
pub struct UpdateChat;

/// Update chatroom push setting
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct UpdateChatRequest {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    #[serde(rename = "pushAlert")]
    pub push_alert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct UpdateChatResponse;
