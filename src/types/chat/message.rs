use serde::{Deserialize, Serialize};

use super::{ChatId, LinkId, LogId, MessageId, UserId};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    pub chat_id: ChatId,
    pub link_id: LinkId,
    pub log_id: LogId,
    pub chat_log: ChatLog,
    pub sent_without_seen: bool,
    pub sender_nickname: Option<String>,
    pub notification_read: Option<bool>,
}

pub type ChatType = i32;
pub type ChatReferer = i32;

pub mod ChatRefererType {
    use super::ChatReferer;

    pub const KAKAOI: ChatReferer = 1;
    pub const BOT: ChatReferer = 2;
}

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
    pub message: Option<String>,
    #[serde(rename = "sendAt")]
    pub sent_at: i64,
    pub attachment: String,
    #[serde(rename = "msgId")]
    pub msg_id: MessageId,
    #[serde(rename = "prevId")]
    pub prev_log_id: LogId,
    pub supplement: Option<String>,
    pub referer: Option<ChatReferer>,
}
