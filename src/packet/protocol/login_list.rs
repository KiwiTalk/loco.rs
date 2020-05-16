use serde::{Deserialize, Serialize};

use crate::internal::{agent::Os, net_type::NetType};
use crate::packet::LocoRequest;
use crate::internal::APP_VERSION;
use bytes::BytesMut;
use crate::types::structs::ChatData;

#[derive(Serialize)]
pub struct LoginListRequest {
    #[serde(rename = "duuid")]
    device_uuid: string,
    #[serde(rename = "oauthToken")]
    oauth_token: string,
    #[serde(rename = "appVer", default = "default_app_version")]
    app_version: string,
    os: Os,
    #[serde(rename = "lang", default = "default_language")]
    language: string,
    #[serde(rename = "ntype")]
    net_type: NetType,
    #[serde(rename = "MCCMNC")]
    network_mcc_mnc: string,
    revision: i32,
    #[serde(rename = "rp")]
    revision_data: Option<BytesMut>,
    #[serde(rename = "chatIds")]
    chat_ids: Vec<i64>,
    #[serde(rename = "maxIds")]
    max_ids: Vec<i64>,
    #[serde(rename = "lastTokenId")]
    last_token_id: i64,
    lbk: i32, // TODO: What is lbk?
    #[serde(rename = "bg", default = "default_background_check")]
    background_check: bool,
    #[serde(rename = "prtVer", default = "default_prt_version")]
    prt_version: char, // TODO: What is prt?
    #[serde(rename = "dtype", default = "default_device_type")]
    device_type: i32,
}

fn default_prt_version() -> char {
    '1'
}

fn default_background_check() -> bool {
    false
}

fn default_device_type() -> i32 {
    1
}

fn default_app_version() -> string {
    APP_VERSION
}

fn default_language() -> string {
    "ko"
}

impl From<LoginListRequest> for LocoRequest {
    fn from(packet: LoginListRequest) -> Self {
        packet.into()
    }
}

#[derive(Deserialize)]
pub struct LoginListResponse {
    #[serde(rename = "userId")]
    user_id: i64,
    revision: i32,
    #[serde(rename = "revisionInfo")]
    revision_info: string,
    #[serde(rename = "ltk")]
    open_chat_token: i32,
    #[serde(rename = "chatDatas")]
    chat_data_list: Vec<ChatData>,
}

fn default_cache_expire() -> i32 {
    -1
}