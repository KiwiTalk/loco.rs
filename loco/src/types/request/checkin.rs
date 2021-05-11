use serde::{Deserialize, Serialize};

use crate::types::NetType;

/// Checkin request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Checkin {
    #[serde(rename = "MCCMNC")]
    mccmnc: String,

    #[serde(rename = "appVer")]
    app_version: String,

    #[serde(rename = "countryISO")]
    country_iso: String,

    #[serde(rename = "lang")]
    language: String,

    #[serde(rename = "ntype")]
    net_type: NetType,

    #[serde(rename = "useSub")]
    use_sub_device: bool,

    os: String,
}
