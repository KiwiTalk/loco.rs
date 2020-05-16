use serde::{Deserialize, Serialize};

use crate::internal::{agent::Os, net_type::NetType};
use crate::packet::LocoRequest;
use crate::internal::APP_VERSION;

#[derive(Serialize)]
pub struct CheckInRequest {
    #[serde(rename = "userId")]
    user_id: i64,
    os: Os,
    #[serde(rename = "ntype")]
    net_type: NetType,
    #[serde(rename = "appVer", default = "default_app_version")]
    app_version: string,
    #[serde(rename = "MCCMNC")]
    network_mcc_mnc: string,
    #[serde(rename = "lang", default = "default_language")]
    language: string,
    #[serde(rename = "useSub")]
    use_sub: bool,
}

fn default_app_version() -> string {
    APP_VERSION
}

fn default_language() -> string {
    "ko"
}

impl From<CheckInRequest> for LocoRequest {
    fn from(packet: CheckInRequest) -> Self {
        packet.into()
    }
}

#[derive(Deserialize)]
pub struct CheckInResponse {
    host: Option<String>,
    host6: Option<String>,
    cshost: Option<String>,
    cshost6: Option<String>,
    port: i32,
    csport: i32,
    vsshost: Option<String>,
    vsshost6: Option<String>,
    vssport: i32,
    #[serde(rename = "cacheExpire", default = "default_cache_expire")]
    cache_expire: i32,
}

fn default_cache_expire() -> i32 {
    -1
}