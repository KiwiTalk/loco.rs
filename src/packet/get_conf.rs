use super::LocoRequest;
use crate::internal::agent::Os;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GetConfig {
    #[serde(rename = "MCCMNC")]
    pub network_mcc_mnc: String,
    pub os: Os,
    pub model: String,
}

impl From<GetConfig> for LocoRequest {
    fn from(packet: GetConfig) -> Self {
        Self::GetConfig(packet)
    }
}

pub struct Config {
    pub host_list: Vec<String>,
    pub port_list: Vec<u16>,
    pub revision: usize,
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Ticket {
            lsl: Vec<String>,
        }

        #[derive(Deserialize)]
        struct Wifi {
            ports: Vec<u16>,
        }

        #[derive(Deserialize)]
        struct ConfigRaw {
            ticket: Ticket,
            wifi: Wifi,
            revision: usize,
        }

        let raw = ConfigRaw::deserialize(deserializer)?;
        Ok(Self {
            host_list: raw.ticket.lsl,
            port_list: raw.wifi.ports,
            revision: raw.revision,
        })
    }
}
