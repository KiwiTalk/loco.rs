/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::user::UserVariant;
use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(MemberRequest, MemberResponse)]
pub struct Member;

/// Request detailed members of chatroom.
/// Official client send this when clicking profile on chatroom.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct MemberRequest {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// List of requesting user id list
    #[serde(rename = "memberIds")]
    pub user_ids: Vec<i64>,
}

/// Responses detailed members of chatroom.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct MemberResponse {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// List of requested user list
    #[serde(rename = "members")]
    pub members: Vec<UserVariant>,
}
