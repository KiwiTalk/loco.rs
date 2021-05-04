use serde::{Deserialize, Serialize};
use strum::ToString;

use self::chat::MessagePart;
pub mod chat;

#[derive(Serialize, Deserialize, ToString, Debug, PartialEq, Clone)]
pub enum Method {
    #[strum(serialize = "MSG")]
    Message(Message),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    pub status: Status,
    #[serde(flatten)]
    pub message: MessagePart,
}

pub type Status = i32;
