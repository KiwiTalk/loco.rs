use std::convert::{TryFrom, TryInto};

use bytes::{Buf, BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
use tokio_util::codec::{Decoder, Encoder};

use crate::Error;

use super::{LocoPacket, RawLocoHeader, RawLocoPacket};

pub struct LocoEncoder;

impl<Payload: Serialize> Encoder<LocoPacket<Payload>> for LocoEncoder {
    type Error = Error;

    fn encode(&mut self, item: LocoPacket<Payload>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let packet = RawLocoPacket::try_from(item)?;
        bincode::serialize_into(dst.writer(), &packet)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct LocoDecoder<Payload> {
    prev_header: Option<RawLocoHeader>,
    _phantom: std::marker::PhantomData<Payload>,
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
            let raw_packet = RawLocoPacket {
                header,
                data: src[..data_size].to_vec(),
            };
            Ok(Some(raw_packet.try_into()?))
        } else {
            src.advance(data_size);
            Err(Error::UnsupportedPacketType)
        }
    }
}
