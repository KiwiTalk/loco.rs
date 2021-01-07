/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use loco_derive::{BsonData, LocoRequest};
use serde::{Deserialize, Serialize};

/// Read message in chatroom
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoRequest)]
pub struct NotiRead {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Read message log id
    ///
    /// Official client decrease every unread chat read count till this chat.
    pub watermark: i64,

    /// Openchat link id
    #[serde(rename = "linkId", skip_serializing_if = "Option::is_none")]
    pub link_id: Option<i64>,
}
