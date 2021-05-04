use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::{types::Method, Error, Result};

pub struct LocoPacket {
    pub id: u32,
    pub status: u16,
    pub method: Method,
}

const HEADER_LEN: usize = 22;

struct Header {
    id: u32,
    status: u16,
    method: String,
    data_type: u8,
    data_size: usize,
}

pub struct LocoDecoder {
    current_header: Option<Header>,
}

impl Decoder for LocoDecoder {
    type Item = LocoPacket;

    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if let Some(header) = self.current_header.take() {
            if src.len() >= header.data_size {
                if header.data_type == 0 || header.data_type == 8 {
                    let method = parse_data(&src[..header.data_size], header.method)?;
                    src.advance(header.data_size);
                    src.reserve(HEADER_LEN);
                    Ok(Some(LocoPacket {
                        id: header.id,
                        status: header.status,
                        method,
                    }))
                } else {
                    src.advance(header.data_size);
                    src.reserve(HEADER_LEN);
                    Ok(None)
                }
            } else {
                self.current_header = Some(header);
                Ok(None)
            }
        } else {
            if src.len() >= HEADER_LEN {
                let id = src.get_u32_le();
                let status = src.get_u16_le();
                let method = std::str::from_utf8(dbg!(&src[..11]))
                    .unwrap()
                    .trim_end_matches('\0')
                    .to_string();
                src.advance(11);
                let data_type = src.get_u8();
                let data_size = src.get_u32_le() as usize;
                src.reserve(data_size);
                self.current_header = Some(Header {
                    id,
                    status,
                    method,
                    data_type,
                    data_size,
                });
            }
            Ok(None)
        }
    }
}

pub struct LocoEncoder;

impl Encoder<LocoPacket> for LocoEncoder {
    type Error = Error;

    fn encode(&mut self, item: LocoPacket, dst: &mut BytesMut) -> Result<()> {
        let mut data = Vec::new();
        bson::to_document(&item.method)?.to_writer(&mut data)?;

        let method_name = item.method.to_string();

        dst.reserve(HEADER_LEN + data.len());
        dst.put_u32_le(item.id);
        dst.put_u16_le(item.status);
        let method_name: Vec<u8> = method_name
            .bytes()
            .chain(std::iter::repeat(0))
            .take(11)
            .collect();
        dst.put(&method_name[..]);

        dst.put_u8(0);
        dst.put_u32_le(data.len() as u32);
        dst.put(&data[..]);

        Ok(())
    }
}

fn parse_data(mut data: &[u8], method: String) -> Result<Method> {
    match method.as_str() {
        "MSG" => bson::Document::from_reader(&mut data)
            .and_then(bson::from_document)
            .map(Method::Message)
            .map_err(From::from),
        _ => Err(Error::UnsupportedMethod(method)),
    }
}
