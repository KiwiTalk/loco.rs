pub struct ChatUser {
}

pub struct ClientChatUser {
}

pub struct ClientChannelUser {
}

pub struct ClientUserInfo {
}

pub enum UserType {
    // -999999
    Undefined,
    // -100
    NotFriend,
    // 9
    Deactivated,
    // 100
    Friend,
    // 1000
    OpenProfile,
}
