use serde::{Deserialize, Serialize};

use crate::types::Timestamp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoAttachment {
    #[serde(rename = "k")]
    pub key_path: String,

    pub url: String,
    #[serde(rename = "w")]
    pub width: usize,
    #[serde(rename = "h")]
    pub height: usize,

    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "thumbnailWidth")]
    pub thumbnail_width: Option<usize>,
    #[serde(rename = "thumbnailHeight")]
    pub thumbnail_height: Option<usize>,

    #[serde(rename = "s", alias = "size")]
    pub file_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoAttachment {
    #[serde(rename = "tk")]
    pub key_path: String,

    pub url: String,
    #[serde(rename = "w")]
    pub width: usize,
    #[serde(rename = "h")]
    pub height: usize,

    #[serde(rename = "s", alias = "size")]
    pub file_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileAttachment {
    #[serde(rename = "k")]
    pub key_path: String,

    pub url: String,

    #[serde(rename = "name")]
    pub file_name: String,
    #[serde(rename = "s", alias = "size")]
    pub file_size: Option<usize>,
    #[serde(rename = "expire")]
    pub expired_at: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioAttachment {
    #[serde(rename = "tk")]
    pub path: String,

    pub url: String,

    #[serde(rename = "s", alias = "size")]
    pub file_size: Option<usize>,
}
