use serde::{Deserialize, Serialize};

use crate::{
    client::Request,
    config::{DeviceType, NetType},
    types::{channel::ChannelData, Method},
};

use super::{ChatId, LogId, UserId};

pub type TokenId = i64;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LoginList {
    #[serde(rename = "appVer")]
    pub app_version: String,
    #[serde(rename = "prtVer")]
    pub prt_version: String,
    pub os: String,
    #[serde(rename = "lang")]
    pub language: String,
    #[serde(rename = "duuid")]
    pub device_uuid: String,
    #[serde(rename = "oauthToken")]
    pub oauth_token: String,
    #[serde(rename = "dtype")]
    pub device_type: DeviceType,
    #[serde(rename = "ntype")]
    pub net_type: NetType,
    #[serde(rename = "MCCMNC")]
    pub mccmnc: String,
    pub revision: i32,
    pub rp: Option<()>,
    pub bg: bool,
}

impl From<LoginList> for Method {
    fn from(item: LoginList) -> Self {
        Self::LoginList(item)
    }
}

impl Request for LoginList {
    type Response = LoginListRes;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LChatList {
    pub chat_datas: Vec<ChannelData>,
    pub last_chat_id: ChatId,
    pub last_token_id: TokenId,
    pub mcm_revision: i32,
    pub del_chat_ids: Vec<ChatId>,
    // kc: Vec<unknown>,
    pub ltk: i64,
    pub lbk: i32,
    pub eof: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginListRes {
    #[serde(flatten)]
    pub chat_list: LChatList,
    pub user_id: UserId,
    pub revision: i32,
    pub revision_info: String,
    pub min_log_id: LogId,
    pub sb: i32,
}
