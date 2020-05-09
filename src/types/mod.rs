pub use channel::*;
pub use chatroom::*;
pub use user::*;

mod chatroom;
mod structs;
mod open_chat;
mod user;
mod channel;

pub type LogId = u64;

pub type Timestamp = u64;
