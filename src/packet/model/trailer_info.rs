use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TrailerInfo {
    #[serde(rename = "compRatio", default = "default_compression_ratio")]
    pub compression_ratio: i32,
    #[serde(rename = "compRatioHD", default = "default_compression_ratio_hd")]
    pub compression_ratio_hd: i32,
    #[serde(rename = "concurrentDownLimit", default = "default_concurrent_down_limit")]
    pub concurrent_down_limit: i32,
    #[serde(rename = "concurrentUpLimit", default = "default_concurrent_up_limit")]
    pub concurrent_up_limit: i32,
    #[serde(rename = "contentExpireTime", default = "default_content_expire_time")]
    pub content_expire_time: i32,
    #[serde(rename = "downCheckSize", default = "default_down_check_size")]
    pub down_check_size: i32,
    #[serde(rename = "maxRelaySize", default = "default_max_relay_size")]
    pub max_relay_size: i32,
    #[serde(rename = "resolution", default = "default_resolution")]
    pub resolution: i32,
    #[serde(rename = "resolutionHD", default = "default_resolution_hd")]
    pub resolution_hd: i32,
    #[serde(rename = "tokenExpireTime", default = "default_token_expire_time")]
    pub token_expire_time: i32,
    #[serde(rename = "upMaxSize", default = "default_up_max_size")]
    pub up_max_size: i32,
    #[serde(rename(serialize = "videoTranscodingBitrate", deserialize = "vBitrate"), default = "default_video_transcoding_bitrate")]
    pub video_transcoding_bitrate: i32,
    #[serde(rename(serialize = "videoTranscodingResolution", deserialize = "vResolution"), default = "default_video_transcoding_resolution")]
    pub video_transcoding_resolution: i32,
    #[serde(rename = "videoUpMaxSize", default = "video_up_max_size")]
    pub video_up_max_size: i32,
}

fn default_compression_ratio() -> i32 {
    40
}

fn default_compression_ratio_hd() -> i32 {
    960
}

fn default_concurrent_down_limit() -> i32 {
    2
}

fn default_concurrent_up_limit() -> i32 {
    2
}

fn default_content_expire_time() -> i32 {
    1209600
}

fn default_down_check_size() -> i32 {
    3145728
}

fn default_max_relay_size() -> i32 {
    512000
}

fn default_resolution() -> i32 {
    960
}

fn default_resolution_hd() -> i32 {
    1280
}

fn default_token_expire_time() -> i32 {
    7200
}

fn default_up_max_size() -> i32 {
    10485760
}

fn default_video_transcoding_bitrate() -> i32 {
    2300000
}

fn default_video_transcoding_resolution() -> i32 {
    480
}

fn video_up_max_size() -> i32 {
    21495807
}