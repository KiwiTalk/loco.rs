use std::convert::TryFrom;

use bytes::{Buf, BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
use tokio_util::codec::{Decoder, Encoder};

use crate::{
    types::{request::LocoRequest, response::LocoResponse},
    Error,
};

use super::{LocoPacket, RawLocoHeader, RawLocoPacket};

#[derive(Default)]
pub struct LocoEncoder;

impl<Payload: Serialize> Encoder<LocoPacket<Payload>> for LocoEncoder {
    type Error = Error;

    fn encode(&mut self, item: LocoPacket<Payload>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let packet = RawLocoPacket::try_from(item)?;
        bincode::serialize_into(dst.writer(), &packet)?;
        Ok(())
    }
}

pub struct LocoDecoder<Payload> {
    prev_header: Option<RawLocoHeader>,
    _phantom: std::marker::PhantomData<Payload>,
}

impl<Payload> Default for LocoDecoder<Payload> {
    fn default() -> Self {
        Self {
            prev_header: None,
            _phantom: Default::default(),
        }
    }
}

impl<Payload> LocoDecoder<Payload> {
    pub fn new() -> Self {
        Default::default()
    }
}

const HEADER_LEN: usize = 22;

impl<Payload: DeserializeOwned> Decoder for LocoDecoder<Payload> {
    type Item = LocoPacket<Payload>;

    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let header = if let Some(header) = self.prev_header.take() {
            header
        } else if src.len() < HEADER_LEN {
            src.reserve(HEADER_LEN);
            return Ok(None);
        } else {
            bincode::deserialize_from(src.reader())?
        };

        let data_size = header.data_size as usize;
        if src.len() < data_size {
            src.reserve(data_size);
            self.prev_header = Some(header);
            Ok(None)
        } else if header.data_type == 0 || header.data_type == 8 {
            Ok(Some(LocoPacket::from_raw(header, &src[..data_size])?))
        } else {
            src.advance(data_size);
            Err(Error::UnsupportedPacketType)
        }
    }
}

#[derive(Default)]
pub struct LocoClientCodec {
    encoder: LocoEncoder,
    decoder: LocoDecoder<LocoResponse>,
}

impl Encoder<LocoPacket<LocoRequest>> for LocoClientCodec {
    type Error = Error;

    fn encode(
        &mut self,
        item: LocoPacket<LocoRequest>,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        self.encoder.encode(item, dst)
    }
}

impl Decoder for LocoClientCodec {
    type Item = LocoPacket<LocoResponse>;

    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.decoder.decode(src)
    }
}

#[derive(Default)]
pub struct LocoServerCodec {
    encoder: LocoEncoder,
    decoder: LocoDecoder<LocoRequest>,
}

impl Encoder<LocoPacket<LocoResponse>> for LocoServerCodec {
    type Error = Error;

    fn encode(
        &mut self,
        item: LocoPacket<LocoResponse>,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        self.encoder.encode(item, dst)
    }
}

impl Decoder for LocoServerCodec {
    type Item = LocoPacket<LocoRequest>;

    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.decoder.decode(src)
    }
}
