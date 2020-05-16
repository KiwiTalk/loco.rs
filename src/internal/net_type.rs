use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum NetType {
    WCDMA = 0,
    WIFI = 1,
}