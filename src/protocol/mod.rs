/*
 * Created on Sat Nov 28 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::io::{Read, Write};

mod header;
mod raw_loco_client;

use serde::{Deserialize, Serialize};

pub use header::LocoHeader;
pub use raw_loco_client::RawLocoClient;
pub mod structs;

pub mod booking;
pub mod chat;
pub mod checkin;

mod ping;

pub use ping::Ping;

pub trait BsonData<'a>: Serialize + Deserialize<'a> {}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct RawLocoPacket {
    pub header: LocoHeader,
    pub data: Vec<u8>,
}

pub trait LocoPacket: Encode + Decode {
    const NAME: &'static str;
}

pub trait LocoPacketPair<T: LocoRequest, U: LocoResponse> {
    const NAME: &'static str;
    type Request = T;
    type Response = U;
}

pub trait LocoRequest: LocoPacket {}

pub trait LocoResponse: LocoPacket {}

pub trait Encode {
    fn encode<T: Write>(&self, buffer: &mut T);
}

pub trait Decode {
    fn decode<T: Read>(&self, buffer: &mut T) -> Self;
}
