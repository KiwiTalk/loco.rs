pub use channel::*;
pub use chat_room::*;
pub use user::*;

pub mod channel;
pub mod chat_room;
pub mod open_chat;
pub mod structs;
pub mod user;

pub type LogId = u64;
pub type Timestamp = u64;
