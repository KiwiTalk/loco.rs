/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::user::UserVariant;
use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(GetMemRequest, GetMemResponse)]
pub struct GetMem;

/// Request simplified member list of chatroom.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct GetMemRequest {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,
}

/// Responses simplified member list of chatroom.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct GetMemResponse {
    /// User list
    pub members: Vec<UserVariant>,
}
