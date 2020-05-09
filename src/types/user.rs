pub type UserId = u64;

pub struct User {
}

pub struct ClientChatUser {
}

pub struct ClientChannelUser {
}

pub struct ClientUserInfo {
}

#[derive(Debug, Clone)]
pub enum UserType {
    Undefined = -999999,
    NotFriend = -100,
    Deactivated = 9,
    Friend = 100,
    OpenProfile = 1000,
}
