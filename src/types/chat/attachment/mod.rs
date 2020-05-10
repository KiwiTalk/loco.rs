mod custom;
mod file;
mod sharp;
mod sticker;
mod text;

pub use custom::*;
pub use file::*;
pub use sharp::*;
pub use sticker::*;
pub use text::*;

pub enum ChatAttachment {
    Photo(PhotoAttachment),
    Video(VideoAttachment),
    File(FileAttachment),
    Audio(AudioAttachment),

    Sharp(SharpAttachment),

    Emoticon(EmoticonAttachment),
    AnimatedEmoticon(AnimatedEmoticonAttachment),

    LongText(LongTextAttachment),
    Reply(ReplyAttachment),
}
