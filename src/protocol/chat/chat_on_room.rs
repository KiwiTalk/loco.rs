/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::open_link::OpenLinkUser;
use crate::protocol::structs::user::UserVariant;
use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(ChatOnRoomRequest, ChatOnRoomResponse)]
pub struct ChatOnRoom;

/// Send before opening chatroom window. Notice server the user opening chatroom window.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct ChatOnRoomRequest {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Last chat log id or 0
    pub token: i64,

    /// Openlink token of chatroom if openchat.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub open_token: Option<i32>,
}

/// Contains user info, watermark list.
/// Client can update chatroom information before opening chatroom window.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct ChatOnRoomResponse {
    /// Chatroom id
    #[serde(rename = "c")]
    pub chat_id: i64,

    /// Chatroom type.
    /// Check `structs/chatroom.rs` ChatroomListData chatroom_type for types.
    #[serde(rename = "t")]
    pub chat_type: String,

    /// watermark user ids
    #[serde(rename = "a")]
    pub watermark_user_ids: Vec<i64>,

    /// Chat read count watermark(chat log id) list.
    /// Decrease every chat read count above watermark.
    #[serde(rename = "w")]
    pub watermarks: Vec<i64>,

    /// Chatroom open token if openchat
    #[serde(rename = "otk", skip_serializing_if = "Option::is_none")]
    pub open_token: Option<i32>,

    /// User list. Variant different by chatroom type.
    /// The list may not have every user data, especially non active users.
    /// If chatroom is openchat doesn't contain client user.
    /// See open_link_user instead.
    /// If there are too many users it will be null. See user_ids instead.
    ///
    /// TODO: Figure out the max limit.
    #[serde(rename = "m")]
    pub users: Option<Vec<UserVariant>>,

    /// If there are too many users, server will send this instead.
    /// The list may not have every user data, especially non active users.
    #[serde(rename = "mi")]
    pub user_ids: Option<Vec<i64>>,

    /// Client open link user if openchat
    #[serde(rename = "olu", skip_serializing_if = "Option::is_none")]
    pub open_link_user: Option<OpenLinkUser>,
}
