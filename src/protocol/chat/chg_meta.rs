/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use loco_derive::{LocoResponse, BsonData};
use crate::protocol::structs::chatroom::ChatroomMeta;

/// Sync Chatroom meta update
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct ChgMeta {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chatroom meta item. Update same type meta.
    pub meta: ChatroomMeta

}