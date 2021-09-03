use serde::{Deserialize, Serialize};

use crate::{
    config::CheckinConfig,
    types::{NetType, UserId},
};

/// Checkin request.
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
    pub use_sub_device: bool,

    pub os: String,

    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}

impl Checkin {
    pub fn from_config(config: &impl CheckinConfig) -> Self {
        Self {
            mccmnc: config.mccmnc().into(),
            app_version: config.app_version().into(),
            country_iso: config.country_iso().into(),
            language: config.language().into(),
            net_type: config.net_type(),
            use_sub_device: config.use_sub_device(),
            os: config.agent().into(),
            user_id: None,
        }
    }

    pub fn with_user(self, user_id: Option<UserId>) -> Self {
        Self { user_id, ..self }
    }
}
