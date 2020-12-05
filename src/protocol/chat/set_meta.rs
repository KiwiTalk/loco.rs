/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use loco_derive::{LocoPacketPair, BsonData};
use crate::protocol::structs::chatroom::ChatroomMeta;

#[derive(LocoPacketPair)]
#[loco_packet_pair(SetMetaRequest, SetMetaResponse)]
pub struct SetMeta;

/// Set Chatroom meta
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SetMetaRequest {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Meta type. See `structs/chatroom.rs` ChatroomMetaType for predefined types.
    #[serde(rename = "type")]
    pub meta_type: i8,

    /// Json or String content. Different depending on type.
    pub content: String

}

/// SETMETA response
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SetMetaResponse {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Updated chatroom meta item.
    pub meta: ChatroomMeta

}