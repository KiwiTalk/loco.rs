use crate::types::open_chat::OpenChatLinkType;

// Maps to OpenLinkStruct
#[derive(Debug, Clone)]
pub struct OpenLink {
    link_id: u64,
    token: i32,
    link_name: String,
    link_url: String,
    link_type: OpenChatLinkType,
    owner: OpenMember,
    description: String,
    cover_url: String,
}

// Maps to OpenMemberStruct
#[derive(Debug, Clone)]
pub struct OpenMember {
    user_id: u64,
    nickname: String,
    profile_image_url: String,
    original_profile_image_url: String,
    full_profile_image_url: String,
    member_type: i32,
    open_chat_token: i32,
}
