use serde::{Deserialize, Serialize};
use strum::ToString;

use self::chat::MessagePart;
pub mod chat;

#[derive(Serialize, Deserialize, ToString)]
pub enum Method {
    #[strum(serialize = "MSG")]
    Message(Message),
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(flatten)]
    pub status: Status,
    #[serde(flatten)]
    pub message: MessagePart,
}

pub type Status = i32;
