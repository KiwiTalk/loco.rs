use serde::{Deserialize, Serialize};

use crate::{
    client::{Request, Response},
    config::BookingConfig,
};

use super::Method;

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

impl From<GetConf> for Method {
    fn from(item: GetConf) -> Self {
        Method::GetConf(item)
    }
}

impl Request for GetConf {
    type Response = GetConfRes;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetConfRes {
    #[serde(rename = "revision")]
    pub last_revision: i32,

    #[serde(rename = "3g")]
    pub config_3g: NetworkConfig,
    #[serde(rename = "wifi")]
    pub config_wifi: NetworkConfig,
    #[serde(rename = "ticket")]
    pub ticket_hosts: TicketHosts,
    #[serde(rename = "profile")]
    pub video_profile: VideoProfile,
    #[serde(rename = "etc")]
    pub extra: ExtraConfig,
    pub trailer: Trailer,
    #[serde(rename = "trailer.h")]
    pub trailer_hd: TrailerHD,
}

impl Response for GetConfRes {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NetworkConfig {
    #[serde(rename = "bgKeepItv")]
    pub background_keep_interval: i32,
    #[serde(rename = "bgReconnItv")]
    pub background_reconnect_interval: i32,
    #[serde(rename = "bgPingItv")]
    pub background_ping_interval: i32,
    #[serde(rename = "fgPingItv")]
    pub foreground_ping_interval: i32,
    #[serde(rename = "reqTimeout")]
    pub request_timeout: i32,
    #[serde(rename = "encType")]
    pub encrypt_type: i32,
    #[serde(rename = "connTimeout")]
    pub connection_timeout: i32,
    #[serde(rename = "recvHeaderTimeout")]
    pub header_receive_timeout: i32,
    #[serde(rename = "inSegTimeout")]
    pub in_segment_timeout: i32,
    #[serde(rename = "outSegTimeout")]
    pub out_segment_timeout: i32,
    #[serde(rename = "blockSendBufSize")]
    pub block_send_buffer_size: i32,
    pub ports: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TicketHosts {
    pub ssl: Vec<String>,
    pub v2sl: Vec<String>,
    pub lsl: Vec<String>,
    pub lsl6: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct VideoProfile {
    #[serde(rename = "vBitrate")]
    pub bitrate: i32,
    #[serde(rename = "vResolution")]
    pub resolution: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtraConfig {
    pub write_retry_timeout: i32,
    pub traceroute_host: Vec<String>,
    pub traceroute_host_6: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Trailer {
    #[serde(rename = "tokenExpireTime")]
    pub token_expire_time: i32,
    pub resolution: i32,
    #[serde(rename = "resolutionHD")]
    pub resolution_hd: i32,
    #[serde(rename = "compRatio")]
    pub compress_ratio: i32,
    #[serde(rename = "compRatioHD")]
    pub compress_ratio_hd: i32,
    #[serde(rename = "downMode")]
    pub download_mode: i32,
    #[serde(rename = "concurrentDownLimit")]
    pub concurrent_download_limit: i32,
    #[serde(rename = "concurrentUpLimit")]
    pub concurrent_upload_limit: i32,
    pub max_relay_size: i32,
    #[serde(rename = "downCheckSize")]
    pub download_check_size: i32,
    #[serde(rename = "upMaxSize")]
    pub upload_max_size: i32,
    #[serde(rename = "videoUpMaxSize")]
    pub video_upload_max_size: i32,
    #[serde(rename = "vCodec")]
    pub video_codec: i32,
    #[serde(rename = "vFps")]
    pub video_fps: i32,
    #[serde(rename = "aCodec")]
    pub audio_codec: i32,
    #[serde(rename = "contentExpireTime")]
    pub content_expire_time: i32,
    #[serde(rename = "vResolution")]
    pub video_resolution: i32,
    #[serde(rename = "vBitrate")]
    pub video_bitrate: i32,
    #[serde(rename = "aFrequency")]
    pub audio_frequency: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TrailerHD {
    #[serde(rename = "vResolution")]
    pub video_resolution: i32,
    #[serde(rename = "vBitrate")]
    pub video_bitrate: i32,
    #[serde(rename = "aFrequency")]
    pub audio_frequency: i32,
}
