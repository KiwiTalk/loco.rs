use serde::{Deserialize, Serialize};

pub type KickoutType = i32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Kickout {
    pub reason: KickoutType,
}

impl Kickout {
    pub const CHANGE_SERVER: KickoutType = -2;
    pub const LOGIN_ANOTHER: KickoutType = 0;
    pub const ACCOUNT_DELETED: KickoutType = 1;
}
