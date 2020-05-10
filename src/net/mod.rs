use crate::packet::*;
use tokio_util::codec::{Decoder, Encoder};
use bson::{to_bson, encode_document};
use bytes::{BytesMut, BufMut, buf::ext::BufMutExt};

pub struct LocoCodec {
    packet_id: u32,
}

pub enum EncodeError {
    Io(std::io::Error),
    Bson(bson::EncoderError),
}

impl From<std::io::Error> for EncodeError {
    fn from(inner: std::io::Error) -> Self {
        Self::Io(inner)
    }
}

impl From<bson::EncoderError> for EncodeError {
    fn from(inner: bson::EncoderError) -> Self {
        Self::Bson(inner)
    }
}

impl Encoder<LocoRequest> for LocoCodec {
    type Error = EncodeError;
    fn encode(&mut self, item: LocoRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}

pub enum DecodeError {
    Io(std::io::Error),
    Bson(bson::DecoderError),
}

impl From<std::io::Error> for DecodeError {
    fn from(inner: std::io::Error) -> Self {
        Self::Io(inner)
    }
}

impl From<bson::DecoderError> for DecodeError {
    fn from(inner: bson::DecoderError) -> Self {
        Self::Bson(inner)
    }
}

impl Decoder for LocoCodec {
    type Item = LocoResponse;
    type Error = DecodeError;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
