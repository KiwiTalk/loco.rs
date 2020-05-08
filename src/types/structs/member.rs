use crate::types::user::UserType;
use crate::types::open_chat::OpenChatMemberType;

// Maps to MemberStruct
pub struct Member {
    user_id: u64,
    nickname: String,
    profile_image_url: String,
    original_profile_image_url: String,
    full_profile_image_url: String,
    user_type: UserType,
    account_id: i32,
    linked_service: String,
    status_message: String,
    open_profile_token: i32,
    open_chat_member_type: OpenChatMemberType,
    profile_link_id: u64,
}
