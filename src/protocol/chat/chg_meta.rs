/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::chatroom::ChatroomMeta;
use loco_derive::{BsonData, LocoResponse};
use serde::{Deserialize, Serialize};

/// Sync Chatroom meta update
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct ChgMeta {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chatroom meta item. Update same type meta.
    pub meta: ChatroomMeta,
}
