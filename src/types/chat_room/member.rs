use serde::{Deserialize, Serialize};

use crate::types::{UserId, UserType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    #[serde(rename = "userId")]
    user_id: UserId,
    #[serde(rename = "type", default)]
    user_type: UserType,

    #[serde(rename = "nickName")]
    nickname: String,

    #[serde(rename = "pi", alias = "profileImageUrl")]
    profile_image_url: Option<String>,
    #[serde(rename = "opi", alias = "originalProfileImageUrl")]
    original_profile_image_url: Option<String>,
    #[serde(rename = "fpi", alias = "fullProfileImageUrl")]
    full_profile_image_url: Option<String>,

    #[serde(rename = "accountId")]
    account_id: Option<i32>,
    // TODO: what is `account_id` format?
    #[serde(rename = "linkedService")]
    linked_service: Option<String>,
    #[serde(rename = "statusMessage")]
    status_message: Option<String>,

    #[serde(rename = "opt")]
    open_profile_token: Option<i32>,
    #[serde(rename = "mt", default)]
    open_chat_member_type: OpenChatMemberType,
    #[serde(rename = "pli")]
    profile_link_id: Option<i64>,
    // TODO: what is `profile_link_id` format?
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpenChatMemberType {
    Unknown = 1,
    None = 2,
    Manager = 4,
}

impl Default for OpenChatMemberType {
    fn default() -> Self {
        OpenChatMemberType::Unknown
    }
}
