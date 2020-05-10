mod protocol_info;
mod get_conf;
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum LocoRequest {
    GetConfig(get_conf::GetConfig),
}

impl LocoRequest {
    fn discriminant(&self) -> &'static str {
        use LocoRequest::*;

        match self {
            GetConfig(_) => "GETCONF",
        }
    }
}

use bson::{decode_document, from_bson};
use bytes::buf::BufExt;

pub enum LocoResponse {
    Config(get_conf::Config),
}

pub enum DecodeError<'a> {
    Bson(bson::DecoderError),
    InvalidDiscriminant(&'a str),
}

impl From<bson::DecoderError> for DecodeError<'_> {
    fn from(inner: bson::DecoderError) -> Self {
        DecodeError::Bson(inner)
    }
}

impl LocoResponse {
    pub(crate) fn from_bson<'a>(discriminant: &'a str, buffer: &mut bytes::BytesMut) -> Result<Self, DecodeError<'a>> {
        let mut reader = buffer.reader();
        match discriminant {
            "GETCONF" => decode_document(&mut reader).map(Into::into).and_then(from_bson).map(Self::Config).map_err(Into::into),
            _ => Err(DecodeError::InvalidDiscriminant(discriminant))
        }
    }
}
