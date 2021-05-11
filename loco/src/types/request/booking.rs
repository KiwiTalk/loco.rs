use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetConf {
    #[serde(rename = "MCCMNC")]
    mccmnc: String,
    model: String,
    os: String,
}
