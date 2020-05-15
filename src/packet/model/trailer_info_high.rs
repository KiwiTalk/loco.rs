#[derive(Deserialize)]
pub struct TrailerInfoHigh {
    #[serde(rename = "videoTranscodingBitrate")]
    pub video_transcoding_bitrate: i32,
    #[serde(rename = "videoTranscodingResolution")]
    pub video_transcoding_resolution: i32,
}