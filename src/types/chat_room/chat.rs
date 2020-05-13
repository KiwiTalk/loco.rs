use serde::{Deserialize, Serialize};

use crate::types::{Channel, ChatAttachment, LogId, Timestamp, User, UserId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatType {
    Feed = 0,
    Text = 1,
    Photo = 2,
    Video = 3,
    Contact = 4,
    Audio = 5,
    DitemEmoticon = 6,
    DitemGift = 7,
    DitemImg = 8,
    KakaoLink = 9,
    Avatar = 11,
    Sticker = 12,
    Schedule = 13,
    Vote = 14,
    Lottery = 15,
    Location = 16,
    Profile = 17,
    File = 18,
    StickerAni = 20,
    Nudge = 21,
    Actioncon = 22,
    Search = 23,
    Reply = 26,
    MultiPhoto = 27,
    Mvoip = 51,
    Custom = 71,
    PlusFriend = 81,
    PlusFriendViral = 83,
    Template = 90,
    ApiTemplate = 91,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mention {
    pub user_id: UserId,
    #[serde(rename = "len")]
    pub length: u32,
    #[serde(rename = "at")]
    pub index_list: Vec<i32>,
}

pub struct Chat {
    pub kind: ChatKind,
    pub log_id: LogId,
    pub prev_log_id: LogId,
    pub channel: Channel,
    pub sender: User,
    pub counter: i32,
    pub text: String,
    pub attachments: Vec<ChatAttachment>,
    pub mentions: Vec<Mention>,
    pub send_at: Timestamp,
}

impl Chat {
    pub fn get_mentions(&self, user_id: UserId) -> Vec<&Mention> {
        self.mentions
            .iter()
            .filter(|mention| mention.user_id == user_id)
            .collect()
    }

    pub fn is_mentioned(&self, user_id: UserId) -> bool {
        !self.get_mentions(user_id).is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum ChatKind {
    Feed(FeedChat),
    Text(TextChat),
    LongText(LongTextChat),
    Photo(PhotoChat),
    MultiPhoto(MultiPhotoChat),
    StaticEmoticon(StaticEmoticonChat),
    AnimatedEmoticon(AnimatedEmoticonChat),
    SharpSearch(SharpSearchChat),
    Reply(ReplyChat),
    KakaoLinkV2(KakaoLinkV2Chat),
}

#[derive(Debug, Clone)]
pub struct FeedChat {}

#[derive(Debug, Clone)]
pub struct TextChat {}

#[derive(Debug, Clone)]
pub struct LongTextChat {}

#[derive(Debug, Clone)]
pub struct PhotoChat {}

#[derive(Debug, Clone)]
pub struct MultiPhotoChat {}

#[derive(Debug, Clone)]
pub struct StaticEmoticonChat {}

#[derive(Debug, Clone)]
pub struct AnimatedEmoticonChat {}

#[derive(Debug, Clone)]
pub struct VideoChat {}

#[derive(Debug, Clone)]
pub struct SharpSearchChat {}

#[derive(Debug, Clone)]
pub struct ReplyChat {}

#[derive(Debug, Clone)]
pub struct KakaoLinkV2Chat {}
