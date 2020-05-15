use crate::packet::*;
use bson::{encode_document, to_bson};
use bytes::{buf::*, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct LocoCodec;

// TODO: implement Display (for human-friendly error message)
#[derive(Debug)]
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

impl Encoder<LocoPacket<LocoRequest>> for LocoCodec {
    type Error = EncodeError;
    fn encode(
        &mut self,
        item: LocoPacket<LocoRequest>,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        let body_buf = BytesMut::new();
        let mut writer = body_buf.writer();
        encode_document(
            &mut writer,
            to_bson(&item.kind)?
                .as_document()
                .expect("Invalid serialization"),
        )?;
        let body_buf = writer.into_inner();
        dst.reserve(22 + body_buf.len());
        dst.put_u32_le(item.packet_id);
        dst.put_u16_le(item.status_code);
        let pad = 10 - item.kind.discriminant().len();
        dst.put_slice(item.kind.discriminant());
        dst.put_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0][..pad]);
        dst.put_u8(item.body_type);
        dst.put_u32_le(body_buf.len() as u32);
        dst.put(body_buf);
        Ok(())
    }
}

// TODO: implement Display (for human-friendly error message)
#[derive(Debug)]
pub enum DecodeError {
    Io(std::io::Error),
    Bson(bson::DecoderError),
    UnknownFormat,
    InvalidDiscriminant(Box<[u8]>),
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

impl From<crate::packet::DecodeError<'_>> for DecodeError {
    fn from(inner: crate::packet::DecodeError) -> Self {
        match inner {
            crate::packet::DecodeError::Bson(bson) => Self::Bson(bson),
            crate::packet::DecodeError::InvalidDiscriminant(discriminant) => {
                Self::InvalidDiscriminant(discriminant.into())
            }
        }
    }
}

impl Decoder for LocoCodec {
    type Item = LocoPacket<LocoResponse>;
    type Error = DecodeError;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 22 {
            src.reserve(22);
            return Ok(None);
        }
        // unwrap is safe because src.len() >= 22
        let mut body_len_buf = src.get(18..22).unwrap();
        let body_len = body_len_buf.get_u32_le() as usize;
        if src.len() < 22 + body_len {
            src.reserve(body_len);
            return Ok(None);
        }
        let packet_id = src.get_u32_le();
        let status_code = src.get_u16_le();
        let discriminant = match src.get(..10) {
            Some(bytes) => {
                let null = bytes.iter().position(|b| *b == 0).unwrap_or(10);
                Box::from(&bytes[..null])
            }
            None => return Err(DecodeError::UnknownFormat),
        };
        let body_type = src.get_u8();
        // body_len is already read
        src.advance(4);
        let body_buf = src.get(..body_len).unwrap();
        let response = LocoResponse::from_bson(&discriminant, body_buf)?;
        Ok(Some(LocoPacket {
            packet_id,
            status_code,
            body_type,
            kind: response,
        }))
    }
}
