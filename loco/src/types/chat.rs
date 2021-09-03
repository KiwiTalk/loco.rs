use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::Nonexhaustive;

use super::UserId;

pub type ChatId = i64;
pub type LogId = i64;
pub type MessageId = i64;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ChatLog {
    #[serde(rename = "logId")]
    pub log_id: LogId,
    #[serde(rename = "chatId")]
    pub chat_id: ChatId,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    #[serde(rename = "authorId")]
    pub sender_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "sendAt")]
    pub sent_at: i64,
    #[serde(rename = "prevId", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
    #[serde(rename = "msgId")]
    pub msg_id: MessageId,
    #[serde(rename = "prevId", skip_serializing_if = "Option::is_none")]
    pub prev_log_id: Option<LogId>,

    pub referer: Option<Nonexhaustive<ChatReferer>>,
}

pub type ChatType = i32;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(i32)]
pub enum ChatReferer {
    KakaoI = 1,
    Bot = 2,
}
