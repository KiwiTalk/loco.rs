/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use loco_derive::{LocoResponse, BsonData};

/// Message read by someone
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct DecunRead {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Read user id
    #[serde(rename = "userId")]
    pub user_id: i64,

    /// Read message log id
    ///
    /// Official client decrease every unread chat read count till this chat.
    pub watermark: i64

}