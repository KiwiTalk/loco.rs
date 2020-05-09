use serde::{Serialize, Deserialize};
use super::super::{ChatKind, Mention, LogId, super::user::UserId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LongTextAttachment {
    #[serde(rename = "path")]
    text_path: String,
    #[serde(rename = "k")]
    key_path: String,
    #[serde(rename = "s", alias = "size")]
    text_size: Option<usize>,
    #[serde(rename = "sd")]
    sd: Option<bool>, // TODO: what is `sd`?
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplyAttachment {
    #[serde(rename = "src_type")]
    source_type: ChatKind,
    #[serde(rename = "src_logId")]
    source_log_id: LogId,
    #[serde(rename = "src_userId")]
    source_user_id: UserId,
    #[serde(rename = "src_message")]
    source_message: String,
    #[serde(rename = "src_mentions")]
    source_mentions: Vec<Mention>,
    #[serde(rename = "src_linkId")]
    source_link_id: Option<u64>, // TODO: what is `link_id`?
}
