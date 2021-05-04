use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MessagePart {
    pub chat_id: i64,
    pub link_id: i64,
    pub log_id: i64,
    pub chat_log: ChatLog,
    pub sent_without_seen: bool,
    pub sender_nickname: Option<String>,
    pub notification_read: Option<bool>,
}

pub type ChatType = i32;
pub type ChatReferer = i32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ChatLog {
    pub log_id: i64,
    pub chat_id: i64,
    pub chat_type: ChatType,
    pub sender_id: i64,
    pub message: String,
    pub sent_at: i64,
    pub attachment: String,
    pub msg_id: i64,
    pub prev_log_id: i64,
    pub supplement: String,
    pub referer: ChatReferer,
}
