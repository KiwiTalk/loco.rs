mod booking;
mod checkin;

use serde::{Deserialize, Serialize};
use strum::ToString;

pub use booking::*;
pub use checkin::*;

pub type DataStatus = i32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum LocoResponse {
    Fail {
        status: DataStatus,
    },
    Success {
        status: DataStatus,
        #[serde(flatten)]
        kind: Box<ResponseKind>,
    },
}

#[derive(Serialize, Deserialize, ToString, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum ResponseKind {
    #[strum(serialize = "GETCONF")]
    GetConf(GetConf),
    #[strum(serialize = "CHECKIN")]
    Checkin(Checkin),
}

impl From<GetConf> for ResponseKind {
    fn from(get_conf: GetConf) -> Self {
        Self::GetConf(get_conf)
    }
}

impl From<Checkin> for ResponseKind {
    fn from(checkin: Checkin) -> Self {
        Self::Checkin(checkin)
    }
}

impl LocoResponse {
    pub const SUCCESS: DataStatus = 0;
    pub const INVALID_USER: DataStatus = -1;
    pub const CLIENT_ERROR: DataStatus = -200;
    pub const NOT_LOGON: DataStatus = -201;
    pub const INVALID_METHOD: DataStatus = -202;
    pub const INVALID_PARAMETER: DataStatus = -203;
    pub const INVALID_BODY: DataStatus = -203;
    pub const INVALID_HEADER: DataStatus = -204;
    pub const UNAUTHORIZED_CHAT_DELETE: DataStatus = -210;
    pub const MEDIA_SERVER_ERROR: DataStatus = -300;
    pub const CHAT_SPAM_LIMIT: DataStatus = -303;
    pub const RESTRICTED_APP: DataStatus = -304;
    pub const LOGINLIST_CHATLIST_FAILED: DataStatus = -305;
    pub const MEDIA_NOT_FOUND: DataStatus = -306;
    pub const MEDIA_THUMB_GEN_FAILED: DataStatus = -307;
    pub const UNSUPPORTED: DataStatus = -308;
    pub const PARTIAL: DataStatus = -310;
    pub const LINK_JOIN_TPS_EXCEEDED: DataStatus = -312;
    pub const CHAT_SEND_RESTRICTED: DataStatus = -321;
    pub const CHANNEL_CREATE_TEMP_RESTRICTED: DataStatus = -322;
    pub const CHANNEL_CREATE_RESTRICTED: DataStatus = -323;
    pub const OPENLINK_UNAVAILABLE: DataStatus = -324;
    pub const INVITE_COUNT_LIMITED: DataStatus = -325;
    pub const OPENLINK_CREATE_RESTRICTED: DataStatus = -326;
    pub const INVALID_CHANNEL: DataStatus = -401;
    pub const CHAT_BLOCKED_BY_FRIEND: DataStatus = -402;
    pub const NOT_CHATABLE_USER: DataStatus = -403;
    pub const GAME_MESSAGE_BLOCKED: DataStatus = -406;
    pub const BLOCKED_IP: DataStatus = -444;
    pub const BACKGROUND_LOGIN_BLOCKED: DataStatus = -445;
    pub const OPERATION_DENIED: DataStatus = -500;
    pub const CHANNEL_USER_LIMITED: DataStatus = -501;
    pub const WRITE_WHILE_BLOCKED: DataStatus = -814;
    pub const OPENCHAT_REJOIN_REQUIRED: DataStatus = -815;
    pub const OPENCHAT_TIME_RESTRICTED: DataStatus = -819;
    pub const INVALID_ACCESS_TOKEN: DataStatus = -950;
    pub const BLOCKED_ACCOUNT: DataStatus = -997;
    pub const AUTH_REQUIRED: DataStatus = -998;
    pub const UPDATE_REQUIRED: DataStatus = -999;
    pub const SERVER_UNDER_MAINTENANCE: DataStatus = -9797;
}
