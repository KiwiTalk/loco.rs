pub use custom::*;
pub use emoticon::*;
pub use file::*;
pub use sharp::*;
pub use text::*;

pub mod custom;
pub mod emoticon;
pub mod file;
pub mod sharp;
pub mod text;

pub enum ChatAttachment {
    Emoticon(EmoticonAttachment),

    Photo(PhotoAttachment),
    Video(VideoAttachment),
    File(FileAttachment),
    Audio(AudioAttachment),

    Sharp(SharpAttachment),

    LongText(LongTextAttachment),
    Reply(ReplyAttachment),
}
