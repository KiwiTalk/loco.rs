use bson::{decode_document, from_bson};
use bytes::buf::BufExt;
use serde::Serialize;

use crate::packet::protocol_info::ProtocolInfo;

mod protocol_info;
mod protocol;

mod model;

pub struct LocoPacket<T> {
    pub packet_id: u32,
    pub status_code: u16,
    pub body_type: u8,
    pub kind: T,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum LocoRequest {
    None,
    GetConfig(protocol::get_conf::GetConfigRequest),
    Ping,
}

impl LocoRequest {
    pub fn discriminant(&self) -> &[u8] {
        use LocoRequest::*;

        match self {
            None => ProtocolInfo::None.as_bytes(),
            GetConfig(_) => ProtocolInfo::GetConfig.as_bytes(),
            Ping => ProtocolInfo::Ping.as_bytes(),
        }
    }
}

pub enum LocoResponse {
    None,
    GetConfig(protocol::get_conf::GetConfigResponse),
    BuyCallServer(protocol::buy_call_server::BuyCallServerResponse),
    Ping,
}

pub(crate) enum DecodeError<'a> {
    Bson(bson::DecoderError),
    InvalidDiscriminant(&'a [u8]),
}

impl From<bson::DecoderError> for DecodeError<'_> {
    fn from(inner: bson::DecoderError) -> Self {
        DecodeError::Bson(inner)
    }
}

impl LocoResponse {
    pub(crate) fn from_bson<'a>(discriminant: &'a [u8], buffer: &[u8]) -> Result<Self, DecodeError<'a>> {
        let mut reader = buffer.reader();
        match ProtocolInfo::from_bytes(discriminant)? {
            ProtocolInfo::None => Ok(LocoResponse::None),
            ProtocolInfo::GetConfig => decode_document(&mut reader)
                .map(Into::into)
                .and_then(from_bson)
                .map(LocoResponse::GetConfig)
                .map_err(Into::into),
            ProtocolInfo::BuyCallServer => decode_document(&mut reader)
                .map(Into::into)
                .and_then(from_bson)
                .map(LocoResponse::BuyCallServer)
                .map_err(Into::into),
            ProtocolInfo::NetworkTest => todo!(),
            ProtocolInfo::CheckIn => todo!(),
            ProtocolInfo::Down => todo!(),
            ProtocolInfo::Mini => todo!(),
            ProtocolInfo::Complete => todo!(),
            ProtocolInfo::Post => todo!(),
            ProtocolInfo::SyncPost => todo!(),
            ProtocolInfo::MultiPost => todo!(),
            ProtocolInfo::AddMember => todo!(),
            ProtocolInfo::NewMember => todo!(),
            ProtocolInfo::Leave => todo!(),
            ProtocolInfo::DeleteMember => todo!(),
            ProtocolInfo::Left => todo!(),
            ProtocolInfo::BlockSync => todo!(),
            ProtocolInfo::BlockAddItem => todo!(),
            ProtocolInfo::BlockDeleteItem => todo!(),
            ProtocolInfo::BlockSpam => todo!(),
            ProtocolInfo::BlockSpams => todo!(),
            ProtocolInfo::BlockMember => todo!(),
            ProtocolInfo::Ship => todo!(),
            ProtocolInfo::MultiShip => todo!(),
            ProtocolInfo::GetTrailer => todo!(),
            ProtocolInfo::Invoice => todo!(),
            ProtocolInfo::MultiInvoice => todo!(),
            ProtocolInfo::MultiCheckTokens => todo!(),
            ProtocolInfo::Create => todo!(),
            ProtocolInfo::PublicCreate => todo!(),
            ProtocolInfo::ChatInfo => todo!(),
            ProtocolInfo::ChatOff => todo!(),
            ProtocolInfo::ChatOnRoom => todo!(),
            ProtocolInfo::GetMeta => todo!(),
            ProtocolInfo::SetMeta => todo!(),
            ProtocolInfo::ChangeMeta => todo!(),
            ProtocolInfo::GetMetas => todo!(),
            ProtocolInfo::GetMCMeta => todo!(),
            ProtocolInfo::SetMCMeta => todo!(),
            ProtocolInfo::ChangeMCMeta => todo!(),
            ProtocolInfo::GetChatST => todo!(),
            ProtocolInfo::SetChatST => todo!(),
            ProtocolInfo::ChangePlusChatStatus => todo!(),
            ProtocolInfo::UpdateChat => todo!(),
            ProtocolInfo::GetMember => todo!(),
            ProtocolInfo::Member => todo!(),
            ProtocolInfo::Write => todo!(),
            ProtocolInfo::Message => todo!(),
            ProtocolInfo::Forward => todo!(),
            ProtocolInfo::DecreaseUnread => todo!(),
            ProtocolInfo::ClearNotification => todo!(),
            ProtocolInfo::ClearBadge => todo!(),
            ProtocolInfo::MChatLogs => todo!(),
            ProtocolInfo::SyncMessage => todo!(),
            ProtocolInfo::DeleteMessage => todo!(),
            ProtocolInfo::SyncDeleteMessage => todo!(),
            ProtocolInfo::SelfDeleteMessage => todo!(),
            ProtocolInfo::SelfDLAMessage => todo!(),
            ProtocolInfo::LoginList => todo!(),
            ProtocolInfo::LocoChatList => todo!(),
            ProtocolInfo::ChangeServer => todo!(),
            ProtocolInfo::Ping => Ok(LocoResponse::Ping),
            ProtocolInfo::VoiceEvent => todo!(),
            ProtocolInfo::SecretCreate => todo!(),
            ProtocolInfo::SecretWrite => todo!(),
            ProtocolInfo::SecretAddMember => todo!(),
            ProtocolInfo::SetPublicKey => todo!(),
            ProtocolInfo::SetSecretKey => todo!(),
            ProtocolInfo::GetPublicKey => todo!(),
            ProtocolInfo::GetSecretKey => todo!(),
            ProtocolInfo::GetLocoPublicKey => todo!(),
            ProtocolInfo::CreateLink => todo!(),
            ProtocolInfo::DeleteLink => todo!(),
            ProtocolInfo::JoinLink => todo!(),
            ProtocolInfo::JoinInfo => todo!(),
            ProtocolInfo::InfoLink => todo!(),
            ProtocolInfo::SyncLink => todo!(),
            ProtocolInfo::UpdateLinkProfile => todo!(),
            ProtocolInfo::KickLeave => todo!(),
            ProtocolInfo::UpdateLink => todo!(),
            ProtocolInfo::RepoLeave => todo!(),
            ProtocolInfo::SyncMainProfile => todo!(),
            ProtocolInfo::SyncLinkCreated => todo!(),
            ProtocolInfo::SyncLinkUpdated => todo!(),
            ProtocolInfo::SyncLinkDeleted => todo!(),
            ProtocolInfo::KickMember => todo!(),
            ProtocolInfo::ReportMember => todo!(),
            ProtocolInfo::LinkDeleted => todo!(),
            ProtocolInfo::SyncLinkProfile => todo!(),
            ProtocolInfo::Kicked => todo!(),
            ProtocolInfo::SyncJoin => todo!(),
            ProtocolInfo::Feed => todo!(),
            ProtocolInfo::CheckJoin => todo!(),
            ProtocolInfo::Blind => todo!(),
            ProtocolInfo::SyncBlind => todo!(),
            ProtocolInfo::ReportLink => todo!(),
            ProtocolInfo::KLSync => todo!(),
            ProtocolInfo::KLDeleteItem => todo!(),
            ProtocolInfo::React => todo!(),
            ProtocolInfo::ReactCount => todo!(),
            ProtocolInfo::SetMemberType => todo!(),
            ProtocolInfo::SyncMemberType => todo!(),
            ProtocolInfo::Rewrite => todo!(),
            ProtocolInfo::SyncRewrite => todo!(),
            ProtocolInfo::RelayEvent => todo!(),
            ProtocolInfo::SyncEvent => todo!(),
            ProtocolInfo::NotificationReceiveSync => todo!(),
            ProtocolInfo::ChangeMoimMetas => todo!(),
            ProtocolInfo::GetMoimMeta => todo!(),
            ProtocolInfo::MoimClick => todo!(),
            ProtocolInfo::SetStatus => todo!(),
            ProtocolInfo::PushAck => todo!(),
            ProtocolInfo::SyncPush => todo!(),
            ProtocolInfo::GetToken => todo!(),
        }
    }
}