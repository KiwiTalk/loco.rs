pub use channel::*;
pub use chat_room::*;
pub use user::*;

pub mod chat_room;
pub mod structs;
pub mod open_chat;
pub mod user;
pub mod channel;

pub type LogId = u64;
pub type Timestamp = u64;
