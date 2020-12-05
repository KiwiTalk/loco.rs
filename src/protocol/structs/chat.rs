/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */


use serde::{Serialize, Deserialize};

/// Chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chatlog {

    /// Chatlog id
    #[serde(rename = "logId")]
    pub log_id: i64,

    /// Previous Chatlog id
    #[serde(rename = "prevId", skip_serializing_if = "Option::is_none")]
    pub prev_log_id: Option<i64>,

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chat type
    #[serde(rename = "type")]
    pub chat_type: i8,

    /// Sender id
    #[serde(rename = "authorId")]
    pub author_id: i64,

    /// Message content
    ///
    /// Usually String, but can be json String according to chat type.
    pub message: String,

    /// Send time
    ///
    /// Multiply by 1000 to convert to Unix time
    #[serde(rename = "sendAt")]
    pub send_at: i32,

    /// Attachment content
    ///
    /// Json data. Have contents and extra data according to chat type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,

    /// Used on pluschat.
    ///
    /// * KakaoI = 1
    /// * Bot = 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<i8>,

    /// Used on pluschat.
    ///
    /// Json data like attachment. Having extra pluschat data like quick reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplement: Option<String>,

    /// Unknown id (Client send count??). Don't confuse with log_id.
    #[serde(rename = "msgId")]
    pub msg_id: i32


}