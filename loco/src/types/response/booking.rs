use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetConf {
    /// Last revision.
    #[serde(rename = "revision")]
    pub last_revision: i32,

    /// Network config for high network speed environment
    #[serde(rename = "3g")]
    pub config_3g: NetworkConfig,

    /// Network config for low network speed environment
    #[serde(rename = "wifi")]
    pub config_wifi: NetworkConfig,

    /// Checkin server host list.
    #[serde(rename = "ticket")]
    pub ticket_hosts: TicketHosts,

    /// Video profile.
    #[serde(rename = "profile")]
    pub video_profile: VideoProfile,

    /// Other configuration.
    #[serde(rename = "etc")]
    pub extra: ExtraConfig,
    /// Video / Audio / File related config.
    pub trailer: Trailer,

    /// HD video / audio config.
    #[serde(rename = "trailer.h")]
    pub trailer_hd: TrailerHD,
}
