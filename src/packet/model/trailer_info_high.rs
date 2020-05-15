use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TrailerInfoHigh {
    #[serde(rename(serialize = "videoTranscodingBitrate", deserialize = "vBitrate"), default = "default_video_transcoding_bitrate")]
    pub video_transcoding_bitrate: i32,
    #[serde(rename(serialize = "videoTranscodingResolution", deserialize = "vResolution"), default = "default_video_transcoding_resolution")]
    pub video_transcoding_resolution: i32,
}

fn default_video_transcoding_bitrate() -> i32 {
    8000000
}

fn default_video_transcoding_resolution() -> i32 {
    1080
}