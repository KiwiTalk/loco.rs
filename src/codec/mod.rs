mod crypto;
mod insecure;
mod secure;

use bytes::{Buf, BufMut, BytesMut};
pub use crypto::*;
pub use insecure::*;
pub use secure::*;

use crate::{types::Method, Error, Result};

#[derive(Debug, PartialEq, Clone)]
pub struct LocoCommand {
    pub id: u32,
    pub status: u16,
    pub method: Method,
}

struct CommandHeader {
    id: u32,
    status: u16,
    method: String,
    data_type: u8,
    data_size: usize,
}

const COMMAND_HEADER_LEN: usize = 22;

fn try_encode_command(dst: &mut BytesMut, item: LocoCommand) -> Result<()> {
    let method_name = item.method.to_string();
    let data = write_method(item.method)?;

    dst.reserve(COMMAND_HEADER_LEN + data.len());
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

fn write_method(method: Method) -> Result<Vec<u8>> {
    let document = match method {
        Method::Message(message) => bson::to_document(&message)?,
    };
    let mut data = Vec::new();
    document.to_writer(&mut data)?;
    Ok(data)
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

fn try_decode_command_data(header: CommandHeader, src: &mut BytesMut) -> Result<LocoCommand> {
    if header.data_type == 0 || header.data_type == 8 {
        let method = parse_command(&src[..header.data_size], header.method)?;
        src.advance(header.data_size);
        src.reserve(COMMAND_HEADER_LEN);
        Ok(LocoCommand {
            id: header.id,
            status: header.status,
            method,
        })
    } else {
        src.advance(header.data_size);
        src.reserve(COMMAND_HEADER_LEN);
        Err(Error::InvalidPacketDataType(header.data_type))
    }
}

fn parse_command(mut data: &[u8], method: String) -> Result<Method> {
    match method.as_str() {
        "MSG" => bson::Document::from_reader(&mut data)
            .and_then(bson::from_document)
            .map(Method::Message)
            .map_err(From::from),
        _ => Err(Error::UnsupportedMethod(method)),
    }
}

#[cfg(test)]
mod test {
    use crate::types::{
        chat::{ChatLog, MessagePart},
        Message,
    };

    use super::*;

    #[test]
    fn encode_decode_command_should_be_redundant() {
        let command = LocoCommand {
            id: 0,
            status: 0,
            method: Method::Message(Message {
                status: 0,
                message: MessagePart {
                    chat_id: 0,
                    link_id: 0,
                    log_id: 0,
                    chat_log: ChatLog {
                        log_id: 0,
                        chat_id: 0,
                        chat_type: 0,
                        sender_id: 0,
                        message: "".into(),
                        sent_at: 0,
                        attachment: "".into(),
                        msg_id: 0,
                        prev_log_id: 0,
                        supplement: "".into(),
                        referer: 0,
                    },
                    sent_without_seen: false,
                    sender_nickname: None,
                    notification_read: None,
                },
            }),
        };

        let mut buffer = BytesMut::new();
        try_encode_command(&mut buffer, command.clone()).unwrap();
        let header = try_decode_command_header(&mut buffer).unwrap().unwrap();
        let decoded_command = try_decode_command_data(header, &mut buffer).unwrap();
        assert_eq!(command, decoded_command);
    }
}
