use super::ChatKind;

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

#[derive(Debug, Clone)]
pub struct PhotoAttachment {
    key_path: String,
    width: u32,
    height: u32,
    image_url: String,
    size: u64,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<u32>,
    thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct VideoAttachment {
    key_path: String,
    width: u32,
    height: u32,
    video_url: String,
    size: u64,
}

#[derive(Debug, Clone)]
pub struct FileAttachment {
    key_path: String,
    file_url: String,
    name: String,
    size: u64,
    expire_date: u64,
}

#[derive(Debug, Clone)]
pub struct AudioAttachment {
    key_path: String,
    audio_url: String,
    size: u64,
}

#[derive(Debug, Clone)]
pub struct EmoticonAttachment {
    name: String,
    path: String,
    emoticon_type: String,
    stop_at: i32,
    sound: String,
    width: u32,
    height: u32,
    description: String,
}

#[derive(Debug, Clone)]
pub struct AnimatedEmoticonAttachment {

}

#[derive(Debug, Clone)]
pub struct LongTextAttachment {
    path: String,
    key_path: String,
    size: u64,
    sd: bool,
}

#[derive(Debug, Clone)]
pub struct SharpAttachment {
    question: String,
    redirect_url: String,
    content_type: String,
    image_url: Option<String>,
    image_width: Option<u32>,
    image_height: Option<u32>,
    content_list: Vec<SharpContent>,
}

#[derive(Debug, Clone)]
pub struct SharpContent {
    description: String,
    content_type: String,
    redirect_url: String,
    image_url: Option<String>,
    image_width: Option<u32>,
    image_height: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ReplyAttachment {
    source_type: ChatKind,
    source_log_id: u64, // source_chat_id?
    source_user_id: u64,
    source_message: String,
    source_mention_list: Vec<MentionContent>,
    source_link_id: u64,
}

#[derive(Debug, Clone)]
pub struct MentionContent {
    user_id: u64,
    length: u32,
    index_list: Vec<i32>,
}

pub struct KakaoLinkV2Attachment {

}

pub struct KakaoV2Content {

}

pub struct ChatMention {

}