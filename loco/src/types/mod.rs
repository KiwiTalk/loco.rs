mod booking;
mod chat;
mod openlink;
pub mod request;
pub mod response;
mod user;

pub use booking::*;
pub use chat::*;
pub use openlink::*;
pub use user::*;

pub type NetType = i32;
