use serde::{Deserialize, Serialize};

use crate::internal::agent::Os;
use crate::packet::LocoRequest;
use crate::packet::model::{connection_info, etc_info, ticket_info, trailer_info, trailer_info_high};

#[derive(Serialize)]
pub struct MiniRequest {
    pub u: i64, // TODO: What is u?
    pub k: String, // TODO: What is k?
    pub mm: String, // TODO: What is mm?
    pub nt: i32, // TODO: What is nt?
    pub os: Os,
    pub av: String, // TODO: What is av?
    pub c: i64, // TODO: What is c?
}

impl From<MiniRequest> for LocoRequest {
    fn from(packet: MiniRequest) -> Self {
        packet.into()
    }
}

#[derive(Deserialize)]
pub struct MiniResponse {
    s: i32, // TODO: What is s?
}
