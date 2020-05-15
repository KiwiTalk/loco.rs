use serde::{Deserialize, Serialize};

use crate::internal::agent::Os;
use crate::packet::LocoRequest;
use crate::packet::model::{connection_info, etc_info, ticket_info, trailer_info, trailer_info_high};

#[derive(Serialize)]
pub struct GetConfigRequest {
    #[serde(rename = "MCCMNC")]
    pub network_mcc_mnc: String,
    pub os: Os,
    pub model: String,
}

impl From<GetConfigRequest> for LocoRequest {
    fn from(packet: GetConfigRequest) -> Self {
        packet.into()
    }
}

#[derive(Deserialize)]
pub struct GetConfigResponse {
    revision: usize,
    #[serde(rename = "3g")]
    wcdma: connection_info::ConnectionInfo,
    wifi: connection_info::ConnectionInfo,
    ticket: ticket_info::TicketInfo,
    trailer: trailer_info::TrailerInfo,
    #[serde(rename = "trailer.h")]
    trailer_high: trailer_info_high::TrailerInfoHigh,
    etc: etc_info::EtcInfo,
}
