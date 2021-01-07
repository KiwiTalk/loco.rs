/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::chat::Chatlog;
use loco_derive::{BsonData, LocoResponse};
use serde::{Deserialize, Serialize};

/// Send when new user join.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct NewMem {
    /// Join feed chat.(?)
    #[serde(rename = "chatLog")]
    pub chat_log: Chatlog,
}
