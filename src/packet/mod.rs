mod protocol_info;
mod get_conf;
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
}

impl LocoRequest {
    pub fn discriminant(&self) -> &[u8] {
        use LocoRequest::*;

        match self {
            GetConfig(_) => b"GETCONF",
        }
    }
}

use bson::{decode_document, from_bson};
use bytes::buf::BufExt;

pub enum LocoResponse {
    Config(get_conf::Config),
}

pub(crate) enum DecodeError<'a> {
    Bson(bson::DecoderError),
    InvalidDiscriminant(&'a [u8]),
}

impl From<bson::DecoderError> for DecodeError<'_> {
    fn from(inner: bson::DecoderError) -> Self {
        DecodeError::Bson(inner)
    }
}

impl LocoResponse {
    pub(crate) fn from_bson<'a>(discriminant: &'a [u8], buffer: &[u8]) -> Result<Self, DecodeError<'a>> {
        let mut reader = buffer.reader();
        match discriminant {
            b"GETCONF" => decode_document(&mut reader).map(Into::into).and_then(from_bson).map(Self::Config).map_err(Into::into),
            _ => Err(DecodeError::InvalidDiscriminant(discriminant))
        }
    }
}
