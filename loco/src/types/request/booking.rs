use serde::{Deserialize, Serialize};

use crate::config::BookingConfig;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetConf {
    #[serde(rename = "MCCMNC")]
    pub mccmnc: String,
    pub model: String,
    pub os: String,
}

impl GetConf {
    pub fn from_config(config: &impl BookingConfig) -> Self {
        Self {
            mccmnc: config.mccmnc().into(),
            model: config.device_model().into(),
            os: config.agent().into(),
        }
    }
}
