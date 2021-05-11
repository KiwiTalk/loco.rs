mod insecure;
mod secure;

pub use insecure::*;
pub use secure::*;

use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    types::{
        request::LocoRequest,
        response::{DataStatus, LocoResponse, ResponseKind},
    },
    Error,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct RawLocoPacket {
    #[serde(flatten)]
    header: RawLocoHeader,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct RawLocoHeader {
    id: i32,
    status: i16,
    method: [u8; 11],
    data_type: u8,
    data_size: u32,
}

pub struct LocoPacket<Payload> {
    pub id: i32,
    pub method: String,
    pub payload: Payload,
}

impl<Payload: Serialize> TryFrom<LocoPacket<Payload>> for RawLocoPacket {
    type Error = Error;

    fn try_from(value: LocoPacket<Payload>) -> Result<Self, Self::Error> {
        use std::convert::TryInto;
        let method = value
            .method
            .bytes()
            .chain(std::iter::repeat(0))
            .take(11)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let payload = bson::to_document(&value.payload)?;
        let mut data = Vec::new();
        payload.to_writer(&mut data)?;
        let header = RawLocoHeader {
            id: value.id,
            status: 0,
            method,
            data_type: 0,
            data_size: data.len() as u32,
        };
        Ok(Self { header, data })
    }
}

impl<Payload: DeserializeOwned> LocoPacket<Payload> {
    fn from_raw(header: RawLocoHeader, mut data: &[u8]) -> Result<Self, Error> {
        let method_slice = header.method.split(|byte| *byte == 0).next().unwrap();
        let method = String::from_utf8(method_slice.into()).unwrap();
        let document = bson::Document::from_reader(&mut data)?;
        let payload = bson::from_document(document)?;
        Ok(Self {
            id: header.id,
            method,
            payload,
        })
    }
}

impl LocoPacket<LocoRequest> {
    pub fn from_request(id: i32, req: impl Into<LocoRequest>) -> Self {
        let payload = req.into();
        Self {
            id,
            method: payload.to_string(),
            payload,
        }
    }
}

impl LocoPacket<LocoResponse> {
    pub fn from_success(id: i32, status: DataStatus, res: impl Into<ResponseKind>) -> Self {
        let kind = res.into();
        let method = kind.to_string();
        let payload = LocoResponse::Success {
            status,
            kind: Box::new(kind),
        };
        Self {
            id,
            method,
            payload,
        }
    }

    pub fn from_fail(id: i32, status: DataStatus, method: String) -> Self {
        let payload = LocoResponse::Fail { status };
        Self {
            id,
            method,
            payload,
        }
    }
}
