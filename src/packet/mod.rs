mod get_conf;
mod protocol_info;
use serde::Serialize;

pub struct LocoPacket<T> {
    pub packet_id: u32,
    pub status_code: u16,
    pub body_type: u8,
    pub kind: T,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum LocoRequest {
    GetConfig(get_conf::GetConfig),
    Ping,
}

impl LocoRequest {
    pub fn discriminant(&self) -> &[u8] {
        use LocoRequest::*;

        match self {
            GetConfig(_) => b"GETCONF",
            Ping => b"PING",
        }
    }
}

use bson::{document::Document, from_bson};
use bytes::buf::BufExt;

pub enum LocoResponse {
    Config(get_conf::Config),
    Ping,
}

pub(crate) enum DecodeError<'a> {
    Bson(bson::de::Error),
    InvalidDiscriminant(&'a [u8]),
}

impl From<bson::de::Error> for DecodeError<'_> {
    fn from(inner: bson::de::Error) -> Self {
        DecodeError::Bson(inner)
    }
}

impl LocoResponse {
    pub(crate) fn from_bson<'a>(
        discriminant: &'a [u8],
        buffer: &[u8],
    ) -> Result<Self, DecodeError<'a>> {
        let mut reader = buffer.reader();
        match discriminant {
            b"GETCONF" => Document::from_reader(&mut reader)
                .map(Into::into)
                .and_then(from_bson)
                .map(Self::Config)
                .map_err(Into::into),
            b"PING" => Ok(Self::Ping),
            _ => Err(DecodeError::InvalidDiscriminant(discriminant)),
        }
    }
}
