mod crypto;
mod insecure;
mod secure;

use bytes::{Buf, BufMut, BytesMut};
pub use crypto::*;
pub use insecure::*;
pub use secure::*;
use serde::Serialize;

use crate::{
    types::{DataStatus, LocoData, LocoResponse, Method},
    Error, Result,
};

#[derive(Debug, PartialEq, Clone)]
pub struct LocoPacket {
    pub id: u32,
    pub status: u16,
    pub method: String,
    pub data: LocoData,
}

impl LocoPacket {
    pub fn from_method(id: u32, status: u16, method: impl Into<Method>) -> Self {
        let data = method.into();
        Self {
            id,
            status,
            method: data.to_string(),
            data: LocoData::Request(data),
        }
    }

    pub fn from_response(
        id: u32,
        status: u16,
        method: &str,
        data_status: DataStatus,
        data: impl Serialize,
    ) -> Result<Self> {
        let data = LocoData::Response(LocoResponse {
            status: data_status,
            extra: bson::to_document(&data)?,
        });
        Ok(LocoPacket {
            id,
            status,
            method: method.to_string(),
            data,
        })
    }
}

struct CommandHeader {
    id: u32,
    status: u16,
    method: String,
    data_type: u8,
    data_size: usize,
}

const COMMAND_HEADER_LEN: usize = 22;

fn try_encode_command(dst: &mut BytesMut, item: LocoPacket) -> Result<()> {
    let data = write_data(item.data)?;

    dst.reserve(COMMAND_HEADER_LEN + data.len());
    dst.put_u32_le(item.id);
    dst.put_u16_le(item.status);
    let method_name: Vec<u8> = item
        .method
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

fn write_data(packet: LocoData) -> Result<Vec<u8>> {
    let document = bson::to_document(&packet)?;
    let mut buffer = Vec::new();
    document.to_writer(&mut buffer)?;
    Ok(buffer)
}

fn try_decode_command_header(src: &mut BytesMut) -> Result<Option<CommandHeader>> {
    if src.len() < COMMAND_HEADER_LEN {
        src.reserve(COMMAND_HEADER_LEN);
        Ok(None)
    } else {
        let id = src.get_u32_le();
        let status = src.get_u16_le();
        let method = std::str::from_utf8(&src[..11])
            .unwrap()
            .trim_end_matches('\0')
            .to_string();
        src.advance(11);
        let data_type = src.get_u8();
        let data_size = src.get_u32_le() as usize;
        Ok(Some(CommandHeader {
            id,
            status,
            method,
            data_type,
            data_size,
        }))
    }
}

fn try_decode_command_data(header: CommandHeader, src: &mut BytesMut) -> Result<LocoPacket> {
    if header.data_type == 0 || header.data_type == 8 {
        let data = parse_data(&src[..header.data_size])?;
        src.advance(header.data_size);
        src.reserve(COMMAND_HEADER_LEN);
        Ok(LocoPacket {
            id: header.id,
            status: header.status,
            method: header.method,
            data,
        })
    } else {
        src.advance(header.data_size);
        src.reserve(COMMAND_HEADER_LEN);
        Err(Error::InvalidPacketDataType(header.data_type))
    }
}

fn parse_data(mut data: &[u8]) -> Result<LocoData> {
    let document = bson::Document::from_reader(&mut data)?;
    Ok(bson::from_document(document)?)
}

#[cfg(test)]
mod test {
    use crate::types::chat::{LChatList, LoginList, LoginListRes};

    use super::*;

    #[test]
    fn encode_decode_command_should_be_redundant() {
        let command = LocoPacket::from_method(
            0,
            0,
            LoginList {
                app_version: "".into(),
                prt_version: "".into(),
                os: "".into(),
                language: "".into(),
                device_uuid: "".into(),
                oauth_token: "".into(),
                device_type: 0,
                net_type: 0,
                mccmnc: "".into(),
                revision: 0,
                rp: None,
                bg: false,
            },
        );

        let response = LocoPacket::from_response(
            0,
            0,
            "LOGINLIST",
            LocoResponse::SUCCESS,
            LoginListRes {
                chat_list: LChatList {
                    chat_datas: vec![],
                    last_chat_id: 0,
                    last_token_id: 0,
                    mcm_revision: 0,
                    del_chat_ids: vec![],
                    ltk: 0,
                    lbk: 0,
                    eof: false,
                },
                user_id: 0,
                revision: 0,
                revision_info: "".into(),
                min_log_id: 0,
                sb: 0,
            },
        )
        .unwrap();

        let mut buffer = BytesMut::new();
        try_encode_command(&mut buffer, command.clone()).unwrap();
        let header = try_decode_command_header(&mut buffer).unwrap().unwrap();
        let decoded_command = try_decode_command_data(header, &mut buffer).unwrap();

        try_encode_command(&mut buffer, response.clone()).unwrap();
        let header = try_decode_command_header(&mut buffer).unwrap().unwrap();
        let decoded_response = try_decode_command_data(header, &mut buffer).unwrap();

        assert_eq!(command, decoded_command);
        assert_eq!(response, decoded_response);
    }
}
