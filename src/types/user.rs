use serde::{Deserialize, Serialize};

pub type UserId = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UserType {
    Undefined = -999_999,
    NotFriend = -100,
    Deactivated = 9,
    Friend = 100,
    OpenProfile = 1000,
}

impl Default for UserType {
    fn default() -> Self {
        UserType::Undefined
    }
}

pub struct User {}

pub struct ClientChatUser {}

pub struct ClientChannelUser {}

pub struct ClientUserInfo {}
