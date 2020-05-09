#[derive(Debug, Clone)]
pub struct OpenChatManager {

}

#[derive(Debug, Clone)]
pub enum OpenChatLinkType {
    Profile = 1,
    Chatroom = 2,
}

#[derive(Debug, Clone)]
pub enum OpenChatMemberType {
    Unknown = 1,
    None = 2,
    Manager = 4,
}