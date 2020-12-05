/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use loco_derive::{LocoRequest, BsonData};

/// Write message to chatroom
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoRequest)]
pub struct Write {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chat type
    #[serde(rename = "type")]
    pub chat_type: i8,

    /// Message id
    ///
    /// Client send count??
    #[serde(rename = "msgId")]
    pub msg_id: i32,

    /// Message content
    ///
    /// Usually String, but can be json String according to chat type.
    #[serde(rename = "msg")]
    pub message: String,

    /// If true, server will assume the client read last message.
    #[serde(rename = "noSeen")]
    pub no_seen: bool,

    /// Attachment content
    ///
    /// Json data. Have contents and extra data according to chat type.
    /// Also known as `extra`.
    #[serde(rename = "extra", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,

    /// Used on pluschat.
    ///
    /// Cannot be used to send by normal user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplement: Option<String>,

}