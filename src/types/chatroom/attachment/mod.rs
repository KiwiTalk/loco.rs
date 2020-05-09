pub use emoticon::*;
pub use file::*;
pub use sharp::*;
pub use text::*;

mod emoticon;
mod file;
mod sharp;
mod text;

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
