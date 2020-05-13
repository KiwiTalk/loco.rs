use serde::{Deserialize, Serialize};

use crate::types::{ChannelId, ChatAttachment, ChatType, LogId, Timestamp, UserId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatLog {
    #[serde(rename = "logId")]
    pub log_id: LogId,
    #[serde(rename = "prevId")]
    pub prev_log_id: LogId,

    #[serde(rename = "chatId")]
    pub channel_id: ChannelId,
    #[serde(rename = "authorId")]
    pub sender_id: UserId,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    #[serde(rename = "msgId")]
    pub chat_counter: i32,

    #[serde(rename = "message")]
    pub text: String,
    #[serde(rename = "sendAt")]
    pub send_at: Timestamp,
    #[serde(rename = "attachment")]
    pub attachment: ChatAttachment,
}
