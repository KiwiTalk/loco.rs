use bson::{decode_document, from_bson};
use bytes::buf::BufExt;
use serde::Serialize;

use crate::packet::protocol_info::ProtocolInfo;

mod protocol_info;
mod get_conf;

pub struct LocoPacket<T> {
    pub packet_id: u32,
    pub status_code: u16,
    pub body_type: u8,
    pub kind: T,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum LocoRequest {
    GetConfig(get_conf::GetConfig),
    Ping,
}

impl LocoRequest {
    pub fn discriminant(&self) -> &[u8] {
        use LocoRequest::*;

        match self {
            GetConfig(_) => ProtocolInfo::GetConfig.as_bytes(),
            Ping => ProtocolInfo::Ping.as_bytes(),
        }
    }
}

pub enum LocoResponse {
    Config(get_conf::Config),
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
        match ProtocolInfo::from_bytes(discriminant) {
            Ok(protocol) => match protocol {
                ProtocolInfo::GetConfig => decode_document(&mut reader).map(Into::into).and_then(from_bson).map(Self::Config).map_err(Into::into),
                ProtocolInfo::BuyCS => {}
                ProtocolInfo::NetworkTest => {}
                ProtocolInfo::CheckIn => {}
                ProtocolInfo::Down => {}
                ProtocolInfo::Mini => {}
                ProtocolInfo::Complete => {}
                ProtocolInfo::Post => {}
                ProtocolInfo::SPost => {}
                ProtocolInfo::MPost => {}
                ProtocolInfo::AddMember => {}
                ProtocolInfo::NewMember => {}
                ProtocolInfo::Leave => {}
                ProtocolInfo::DeleteMember => {}
                ProtocolInfo::Left => {}
                ProtocolInfo::BlockSync => {}
                ProtocolInfo::BlockAddItem => {}
                ProtocolInfo::BlockDeleteItem => {}
                ProtocolInfo::BlockSpam => {}
                ProtocolInfo::BlockSpams => {}
                ProtocolInfo::BlockMember => {}
                ProtocolInfo::Ship => {}
                ProtocolInfo::MShip => {}
                ProtocolInfo::GetTrailer => {}
                ProtocolInfo::Invoice => {}
                ProtocolInfo::MInvoice => {}
                ProtocolInfo::MCheckTokens => {}
                ProtocolInfo::Create => {}
                ProtocolInfo::PCreate => {}
                ProtocolInfo::ChatInfo => {}
                ProtocolInfo::ChatOff => {}
                ProtocolInfo::ChatOnRoom => {}
                ProtocolInfo::GetMeta => {}
                ProtocolInfo::SetMeta => {}
                ProtocolInfo::ChangeMeta => {}
                ProtocolInfo::GetMetas => {}
                ProtocolInfo::GetMCMeta => {}
                ProtocolInfo::SetMCMeta => {}
                ProtocolInfo::ChangeMCMeta => {}
                ProtocolInfo::GetChatST => {}
                ProtocolInfo::SetChatST => {}
                ProtocolInfo::ChangeChatST => {}
                ProtocolInfo::UpdateChat => {}
                ProtocolInfo::GetMember => {}
                ProtocolInfo::Member => {}
                ProtocolInfo::Write => {}
                ProtocolInfo::Message => {}
                ProtocolInfo::Forward => {}
                ProtocolInfo::DecreaseUnread => {}
                ProtocolInfo::ClearNotification => {}
                ProtocolInfo::ClearBadge => {}
                ProtocolInfo::MChatLogs => {}
                ProtocolInfo::SyncMessage => {}
                ProtocolInfo::DeleteMessage => {}
                ProtocolInfo::SyncDeleteMessage => {}
                ProtocolInfo::SelfDeleteMessage => {}
                ProtocolInfo::SelfDLAMessage => {}
                ProtocolInfo::LoginList => {}
                ProtocolInfo::LoginChatList => {}
                ProtocolInfo::ChangeServer => {}
                ProtocolInfo::VOEvent => {}
                ProtocolInfo::SCreate => {}
                ProtocolInfo::SWrite => {}
                ProtocolInfo::SAddMember => {}
                ProtocolInfo::SetPublicKey => {}
                ProtocolInfo::SetSecretKey => {}
                ProtocolInfo::GetPublicKey => {}
                ProtocolInfo::GetSecretKey => {}
                ProtocolInfo::GetLdapPublicKey => {}
                ProtocolInfo::CreateLink => {}
                ProtocolInfo::DeleteLink => {}
                ProtocolInfo::JoinLink => {}
                ProtocolInfo::JoinInfo => {}
                ProtocolInfo::InfoLink => {}
                ProtocolInfo::SyncLink => {}
                ProtocolInfo::UpdateLinkProfile => {}
                ProtocolInfo::KickLeave => {}
                ProtocolInfo::UpdateLink => {}
                ProtocolInfo::RepoLeave => {}
                ProtocolInfo::SyncMainProfile => {}
                ProtocolInfo::SyncLinkCR => {}
                ProtocolInfo::SyncLinkUpdate => {}
                ProtocolInfo::SyncLinkDownload => {}
                ProtocolInfo::KickMember => {}
                ProtocolInfo::ReportMember => {}
                ProtocolInfo::LinkKicked => {}
                ProtocolInfo::LinkDeleted => {}
                ProtocolInfo::SyncLinkProfile => {}
                ProtocolInfo::Kicked => {}
                ProtocolInfo::SyncJoin => {}
                ProtocolInfo::Feed => {}
                ProtocolInfo::CheckJoin => {}
                ProtocolInfo::Blind => {}
                ProtocolInfo::SyncBlind => {}
                ProtocolInfo::ReportLink => {}
                ProtocolInfo::KLSync => {}
                ProtocolInfo::KLDeleteItem => {}
                ProtocolInfo::React => {}
                ProtocolInfo::ReactCount => {}
                ProtocolInfo::SetMemberType => {}
                ProtocolInfo::SyncMemberType => {}
                ProtocolInfo::Rewrite => {}
                ProtocolInfo::SyncRewrite => {}
                ProtocolInfo::RelayEvent => {}
                ProtocolInfo::SyncEvent => {}
                ProtocolInfo::GroupAdd => {}
                ProtocolInfo::GroupAddSync => {}
                ProtocolInfo::GroupDelete => {}
                ProtocolInfo::GroupDeleteSync => {}
                ProtocolInfo::GroupUpdate => {}
                ProtocolInfo::GroupUpdateSync => {}
                ProtocolInfo::GroupAddItem => {}
                ProtocolInfo::GroupAddItemSync => {}
                ProtocolInfo::GroupDeleteItem => {}
                ProtocolInfo::GroupDeleteItemSync => {}
                ProtocolInfo::GroupDeleteItemAttr => {}
                ProtocolInfo::GroupDeleteItemAttrSync => {}
                ProtocolInfo::GroupSetPosition => {}
                ProtocolInfo::GroupPositionSync => {}
                ProtocolInfo::GroupList => {}
                ProtocolInfo::NotificationReceiveSync => {}
                ProtocolInfo::ChangeMoimMetas => {}
                ProtocolInfo::GetMoimMeta => {}
                ProtocolInfo::MoimClick => {}
                ProtocolInfo::SetST => {}
                ProtocolInfo::PushAck => {}
                ProtocolInfo::SPush => {}
                ProtocolInfo::GetToken => {}
                ProtocolInfo::Ping => Ok(Self::Ping),
            },
            Err(error) => Err(DecodeError::InvalidDiscriminant(discriminant)),
        }
    }
}