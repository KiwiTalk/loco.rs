pub use channel::*;
pub use chatroom::*;
pub use user::*;

pub mod chatroom;
pub mod structs;
pub mod open_chat;
pub mod user;
pub mod channel;

pub type LogId = u64;
pub type Timestamp = u64;
