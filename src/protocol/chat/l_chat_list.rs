/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::chatroom::ChatroomListData;
use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(LChatListRequest, LChatListResponse)]
pub struct LChatList;

/// Request every chatroom list
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct LChatListRequest {
    /// Known chatroom id list
    #[serde(rename = "chatIds")]
    pub chat_ids: Vec<i64>,

    /// Unknown
    #[serde(rename = "maxIds")]
    pub max_ids: Vec<i64>,

    /// Unknown
    #[serde(rename = "lastTokenId")]
    pub last_token_id: i64,
}

/// Request every chatroom list
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct LChatListResponse {
    #[serde(rename = "chatDatas")]
    pub chat_datas: Vec<ChatroomListData>,
}
