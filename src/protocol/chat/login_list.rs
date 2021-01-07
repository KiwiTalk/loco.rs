/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use crate::protocol::chat::l_chat_list::{LChatListRequest, LChatListResponse};
use crate::protocol::structs::client::ClientInfo;
use loco_derive::{BsonData, LocoPacketPair};
use serde::{Deserialize, Serialize};

#[derive(LocoPacketPair)]
#[loco_packet_pair(LoginListRequest, LoginListResponse)]
pub struct LoginList;

/// Login to loco server
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct LoginListRequest {
    #[serde(flatten)]
    pub client: ClientInfo,

    /// Protocol version, seems like always "1"
    #[serde(rename = "prtVer")]
    pub protocol_version: String,

    /// Device uuid String. Usually hashed unique id.
    #[serde(rename = "duuid")]
    pub device_uuid: String,

    /// OAuth access token
    #[serde(rename = "oauthToken")]
    pub oauth_token: String,

    #[serde(rename = "lang")]
    pub language: String,

    /// Device type (2 for pc)
    #[serde(rename = "dtype")]
    pub device_type: i8,

    /// Unknown
    pub revision: i32,

    /// Unknown. Always None(?)
    pub rp: (),

    #[serde(flatten)]
    pub chat_list: LChatListRequest,

    /// Unknown
    #[serde(rename = "lbk")]
    pub last_block_token: i32,

    /// background checking(?)
    #[serde(rename = "bg")]
    pub background: bool,
}

/// Contains userId, tokens, chatroom list.
/// The purposes of tokens, revisions are unknown yet.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct LoginListResponse {
    /// Logon user id
    #[serde(rename = "userId")]
    pub user_id: i64,

    #[serde(flatten)]
    pub chat_list: LChatListResponse,

    /// Deleted chatroom ids(?)
    #[serde(rename = "delChatIds")]
    pub deleted_chat_ids: Vec<i64>,

    /// Unknown
    pub eof: bool,

    /// Latest chatroom id
    #[serde(rename = "lastChatId")]
    pub last_chat_id: i64,

    /// Latest token(Unknown) id
    #[serde(rename = "lastTokenId")]
    pub last_token_id: i64,

    /// Oldest chat id (?)
    #[serde(rename = "minLogId")]
    pub min_log_id: i64,

    /// Latest token(Unknown)(?)
    #[serde(rename = "ltk")]
    pub last_token: i64,

    /// Latest block token(Unknown)(?)
    #[serde(rename = "lbk")]
    pub last_block_token: i64,

    /// Latest mcm(?) revision
    #[serde(rename = "mcmRevision")]
    pub mcm_revision: i64,

    /// Unknown
    pub revision: i32,

    /// Revision(?) Info (Json)
    #[serde(rename = "revisionInfo")]
    pub revision_info: String,

    /// Unknown
    pub sb: i32,
    // Unknown, Unknown item type
    //pub kc: Vec<()>
}
