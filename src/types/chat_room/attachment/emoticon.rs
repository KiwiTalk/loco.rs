use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmoticonAttachment {
    pub path: String,

    pub name: String,
    #[serde(rename = "type")]
    pub sticker_type: String,
    // TODO: what is `type` format?
    #[serde(rename = "s")]
    pub stop_at: Option<i32>,
    // TODO: what is `stop_at` format?
    pub alt: Option<String>,

    pub width: Option<usize>,
    pub height: Option<usize>,
    pub sound: Option<String>, // TODO: what is `sound` format?
}
