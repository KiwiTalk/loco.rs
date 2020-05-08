#[derive(Debug, Clone)]
pub struct OpenChatManager {

}

#[derive(Debug, Clone)]
pub enum OpenChatLinkType {
    // 1
    Profile,
    // 2
    Chatroom,
}

#[derive(Debug, Clone)]
pub enum OpenChatMemberType {
    // 1
    Unknown,
    // 2
    None,
    // 4
    Manager,
}