use super::ChatKind;
use serde::{Serialize, Deserialize};
use super::user::UserId;


pub enum ChatAttachment {
    Photo(PhotoAttachment),
    Video(VideoAttachment),
    File(FileAttachment),
    Audio(AudioAttachment),
    Emoticon(EmoticonAttachment),
    AnimatedEmoticon(AnimatedEmoticonAttachment),
    LongText(LongTextAttachment),
    Sharp(SharpAttachment),
    Reply(ReplyAttachment),
    KakaoLinkV2(KakaoLinkV2Attachment),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoAttachment {
    #[serde(rename = "k")]
    key_path: String,
    #[serde(rename = "w")]
    image_width: u32,
    #[serde(rename = "h")]
    image_height: u32,
    #[serde(rename = "url")]
    image_url: String,
    #[serde(rename = "s", alias = "size")]
    image_size: u64,
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: Option<String>,
    #[serde(rename = "thumbnailWidth")]
    thumbnail_width: Option<u32>,
    #[serde(rename = "thumbnailHeight")]
    thumbnail_height: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoAttachment {
    #[serde(rename = "tk")]
    key_path: String,
    #[serde(rename = "w")]
    video_width: u32,
    #[serde(rename = "h")]
    video_height: u32,
    #[serde(rename = "url")]
    video_url: String,
    #[serde(rename = "s", alias = "size")]
    video_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileAttachment {
    #[serde(rename = "k")]
    key_path: String,
    #[serde(rename = "name")]
    file_name: String,
    #[serde(rename = "url")]
    file_url: String,
    #[serde(rename = "s", alias = "size")]
    file_size: u64,
    #[serde(rename = "expire")]
    file_expired_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioAttachment {
    #[serde(rename = "tk")]
    key_path: String,
    #[serde(rename = "url")]
    audio_url: String,
    #[serde(rename = "s", alias = "size")]
    audio_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmoticonAttachment {
    #[serde(rename = "path")]
    emoticon_path: String,
    #[serde(rename = "name")]
    emoticon_name: String,
    #[serde(rename = "type")]
    emoticon_type: String,
    #[serde(rename = "s", default = "0")]
    emoticon_stop_at: i32,
    #[serde(rename = "alt")]
    emoticon_alt: Option<String>,
    #[serde(rename = "width")]
    emoticon_width: Option<u32>,
    #[serde(rename = "height")]
    emoticon_height: Option<u32>,
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
    #[serde(rename = "s", default = "0")]
    emoticon_stop_at: i32,
    #[serde(rename = "alt")]
    emoticon_alt: Option<String>,
    #[serde(rename = "width")]
    emoticon_width: Option<u32>,
    #[serde(rename = "height")]
    emoticon_height: Option<u32>,
    #[serde(rename = "sound")]
    emoticon_sound: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LongTextAttachment {
    #[serde(rename = "path")]
    text_path: String,
    #[serde(rename = "k")]
    key_path: String,
    #[serde(rename = "s", alias = "size")]
    text_size: u64,
    #[serde(rename = "sd")]
    sd: bool, // TODO: what is `sd`?
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpAttachment {
    #[serde(rename = "Q")]
    sharp_question: String,
    #[serde(rename = "V")]
    content_type: String,
    // TODO: what is `V`?
    #[serde(rename = "L")]
    sharp_link: String,
    #[serde(rename = "I")]
    image_url: Option<String>,
    #[serde(rename = "W")]
    image_width: Option<u32>,
    #[serde(rename = "H")]
    image_height: Option<u32>,
    #[serde(rename = "R", flatten)]
    content_list: Vec<SharpContent>, // TODO: what is `R`?
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpContent {
    #[serde(rename = "D")]
    description: String,
    #[serde(rename = "T")]
    content_type: String,
    #[serde(rename = "L")]
    sharp_link: String,
    #[serde(rename = "I")]
    image_url: Option<String>,
    #[serde(rename = "W")]
    image_width: Option<u32>,
    #[serde(rename = "H")]
    image_height: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mention {
    pub user_id: UserId,
    #[serde(rename = "len")]
    pub length: u32,
    #[serde(rename = "at")]
    pub index_list: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplyAttachment {
    source_type: ChatKind,
    source_log_id: u64,
    // source_chat_id?
    source_user_id: u64,
    source_message: String,
    source_mention_list: Vec<Mention>,
    source_link_id: u64,
}

pub struct KakaoLinkV2Attachment {}

pub struct KakaoV2Content {}

pub struct ChatMention {}
