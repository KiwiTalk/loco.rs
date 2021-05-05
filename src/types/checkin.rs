use serde::{Deserialize, Serialize};

use crate::{
    client::{Request, Response},
    config::{CheckinConfig, NetType},
};

use super::{chat::UserId, Method};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Checkin {
    #[serde(rename = "MCCMNC")]
    pub mccmnc: String,
    #[serde(rename = "appVer")]
    pub app_version: String,
    #[serde(rename = "countryISO")]
    pub country_iso: String,
    #[serde(rename = "lang")]
    pub language: String,
    #[serde(rename = "ntype")]
    pub net_type: NetType,
    #[serde(rename = "useSub")]
    pub sub_device: bool,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    pub os: String,
}

impl Checkin {
    pub fn from_config(config: &impl CheckinConfig) -> Self {
        Self {
            mccmnc: config.mccmnc().into(),
            app_version: config.app_version().into(),
            country_iso: config.country_iso().into(),
            language: config.language().into(),
            net_type: config.net_type(),
            sub_device: config.sub_device(),
            user_id: None,
            os: config.agent().into(),
        }
    }

    pub fn with_user(self, user_id: Option<UserId>) -> Self {
        Self { user_id, ..self }
    }
}

impl From<Checkin> for Method {
    fn from(item: Checkin) -> Self {
        Method::Checkin(item)
    }
}

impl Request for Checkin {
    type Response = CheckinRes;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CheckinRes {
    pub host: String,
    pub host6: String,
    pub port: u16,
    #[serde(rename = "cacheExpire")]
    pub cache_expire: i32,
    #[serde(rename = "cshost")]
    pub call_server_host: String,
    #[serde(rename = "csport")]
    pub call_server_port: u16,
    #[serde(rename = "cshost6")]
    pub call_server_host6: String,
    #[serde(rename = "vsshost")]
    pub video_server_host: String,
    #[serde(rename = "vssport")]
    pub video_server_port: u16,
    #[serde(rename = "vsshost6")]
    pub video_server_host6: String,
}

impl Response for CheckinRes {}
