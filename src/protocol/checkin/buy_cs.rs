/*
 * Created on Tue Dec 01 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::structs::client::ClientInfo;
use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(BuyCSRequest, BuyCSResponse)]
pub struct BuyCS;

/// Request call server host data.
/// Checkin response already contains call server info
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct BuyCSRequest {
    #[serde(flatten)]
    pub client: ClientInfo,

    #[serde(rename = "countryISO")]
    pub country_iso: String,
}

/// Answer call server information
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct BuyCSResponse {
    /// Call server ip
    #[serde(rename = "cshost")]
    pub cs_host: String,

    /// Call server ip(v6)
    #[serde(rename = "cshost6")]
    pub cs_host6: String,

    /// Call server port
    #[serde(rename = "csport")]
    pub cs_port: i32,

    /// Unknown server ip
    #[serde(rename = "vsshost")]
    pub vss_host: String,

    /// Unknown server ip(v6)
    #[serde(rename = "vsshost6")]
    pub vss_host6: String,

    /// Unknown server port
    #[serde(rename = "vssport")]
    pub vss_port: i32,
}
