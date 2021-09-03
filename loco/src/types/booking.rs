use serde::{Deserialize, Serialize};

/// Network related settings.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NetworkConfig {
    /// Background keep interval.
    #[serde(rename = "bgKeepItv")]
    pub background_keep_interval: i32,

    /// Background reconnect interval.
    #[serde(rename = "bgReconnItv")]
    pub background_reconnect_interval: i32,

    /// Background ping interval.
    #[serde(rename = "bgPingItv")]
    pub background_ping_interval: i32,

    /// Foreground keep interval.
    #[serde(rename = "fgPingItv")]
    pub foreground_ping_interval: i32,

    /// Request timeout.
    #[serde(rename = "reqTimeout")]
    pub request_timeout: i32,

    /// Encrypt type. Used in secure layer.
    #[serde(rename = "encType")]
    pub encrypt_type: i32,

    /// Connection timeout.
    #[serde(rename = "connTimeout")]
    pub connection_timeout: i32,

    /// Packet header receive timeout.
    #[serde(rename = "recvHeaderTimeout")]
    pub header_receive_timeout: i32,

    #[serde(rename = "inSegTimeout")]
    pub in_segment_timeout: i32,

    #[serde(rename = "outSegTimeout")]
    pub out_segment_timeout: i32,

    /// Max tcp packet size. (uncertain)
    #[serde(rename = "blockSendBufSize")]
    pub block_send_buffer_size: i32,

    /// Usable port list.
    pub ports: Vec<i32>,
}

/// Checkin server host list.
/// Only lsl and lsl6 work.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TicketHosts {
    pub ssl: Vec<String>,
    pub v2sl: Vec<String>,
    pub lsl: Vec<String>,
    pub lsl6: Vec<String>,
}

/// Video profile information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct VideoProfile {
    #[serde(rename = "vBitrate")]
    pub bitrate: i32,
    #[serde(rename = "vResolution")]
    pub resolution: i32,
}

/// Miscellaneous settings.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtraConfig {
    pub write_retry_timeout: i32,
    pub traceroute_host: Vec<String>,
    pub traceroute_host_6: Vec<String>,
}

/// Video / Audio / File related config.
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

/// HD video / audio settings.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TrailerHD {
    #[serde(rename = "vResolution")]
    pub video_resolution: i32,
    #[serde(rename = "vBitrate")]
    pub video_bitrate: i32,
    #[serde(rename = "aFrequency")]
    pub audio_frequency: i32,
}
