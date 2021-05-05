use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::{Error, Result};

use super::{
    try_decode_command_data, try_decode_command_header, try_encode_command, CommandHeader,
    LocoPacket,
};

pub struct LocoCodec {
    current_header: Option<CommandHeader>,
}

impl Decoder for LocoCodec {
    type Item = LocoPacket;

    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        let current_header = match self.current_header.take() {
            header @ Some(_) => header,
            None => try_decode_command_header(src)?,
        };

        if let Some(header) = current_header {
            if src.len() >= header.data_size {
                try_decode_command_data(header, src).map(Some)
            } else {
                src.reserve(header.data_size);
                self.current_header = Some(header);
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder<LocoPacket> for LocoCodec {
    type Error = Error;

    fn encode(&mut self, item: LocoPacket, dst: &mut BytesMut) -> Result<()> {
        try_encode_command(dst, item)
    }
}
