mod kickout;
mod login_list;
mod message;

pub use kickout::*;
pub use login_list::*;
pub use message::*;

pub type ChatId = i64;
pub type LinkId = i64;
pub type LogId = i64;
pub type UserId = i64;
pub type MessageId = i64;
