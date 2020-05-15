#[derive(Deserialize)]
pub struct TrailerInfo {
    #[serde(rename = "compRatio")]
    pub compression_ratio: i32,
    #[serde(rename = "compRatioHD")]
    pub compression_ratio_hd: i32,
    #[serde(rename = "concurrentDownLimit")]
    pub concurrent_down_limit: i32,
    #[serde(rename = "concurrentUpLimit")]
    pub concurrent_up_limit: i32,
    #[serde(rename = "contentExpireTime")]
    pub content_expire_time: i32,
    #[serde(rename = "downCheckSize")]
    pub down_check_size: i32,
    #[serde(rename = "maxRelaySize")]
    pub max_relay_size: i32,
    #[serde(rename = "resolution")]
    pub resolution: i32,
    #[serde(rename = "resolutionHD")]
    pub resolution_hd: i32,
    #[serde(rename = "tokenExpireTime")]
    pub token_expire_time: i32,
    #[serde(rename = "upMaxSize")]
    pub up_max_size: i32,
    #[serde(rename = "videoTranscodingBitrate")]
    pub video_transcoding_bitrate: i32,
    #[serde(rename = "videoTranscodingResolution")]
    pub video_transcoding_resolution: i32,
    #[serde(rename = "videoUpMaxSize")]
    pub video_up_max_size: i32,
}