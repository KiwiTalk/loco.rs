use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmoticonAttachment {
    #[serde(rename = "path")]
    emoticon_path: String,
    #[serde(rename = "name")]
    emoticon_name: String,
    #[serde(rename = "type")]
    emoticon_type: String,
    #[serde(rename = "s")]
    emoticon_stop_at: Option<i32>, // TODO: what is `stop_at`
    #[serde(rename = "alt")]
    emoticon_alt: Option<String>,
    #[serde(rename = "width")]
    emoticon_width: Option<usize>,
    #[serde(rename = "height")]
    emoticon_height: Option<usize>,
    #[serde(rename = "sound")]
    emoticon_sound: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimatedEmoticonAttachment {
    #[serde(rename = "path")]
    emoticon_path: String,
    #[serde(rename = "name")]
    emoticon_name: String,
    #[serde(rename = "type")]
    emoticon_type: String,
    #[serde(rename = "s")]
    emoticon_stop_at: Option<i32>, // TODO: what is `stop_at`
    #[serde(rename = "alt")]
    emoticon_alt: Option<String>,
    #[serde(rename = "width")]
    emoticon_width: Option<usize>,
    #[serde(rename = "height")]
    emoticon_height: Option<usize>,
    #[serde(rename = "sound")]
    emoticon_sound: Option<String>,
}
