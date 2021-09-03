use serde::{Deserialize, Serialize};
use strum::ToString;

mod booking;
mod checkin;
mod ping;

pub use booking::*;
pub use checkin::*;
pub use ping::*;

#[derive(Serialize, Deserialize, ToString, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum LocoRequest {
    #[strum(serialize = "PING")]
    Ping(Ping),
    #[strum(serialize = "GETCONF")]
    GetConf(GetConf),
    #[strum(serialize = "CHECKIN")]
    Checkin(Checkin),
}

impl From<Ping> for LocoRequest {
    fn from(ping: Ping) -> Self {
        Self::Ping(ping)
    }
}

impl From<GetConf> for LocoRequest {
    fn from(get_conf: GetConf) -> Self {
        Self::GetConf(get_conf)
    }
}

impl From<Checkin> for LocoRequest {
    fn from(checkin: Checkin) -> Self {
        Self::Checkin(checkin)
    }
}
