use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::OpenlinkChannelUser;

pub type LinkId = i64;

/// Openlink information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Openlink {
    /// Link ID.
    #[serde(rename = "li")]
    link_id: LinkId,

    /// Open token.
    #[serde(rename = "otk")]
    open_token: i32,

    /// Link name.
    #[serde(rename = "ln")]
    name: String,

    /// Link URL.
    #[serde(rename = "lu")]
    url: String,

    /// Link image URL.
    #[serde(rename = "liu")]
    image_url: String,

    /// Link cover URL.
    #[serde(rename = "lcu")]
    cover_url: String,

    /// Owner of this channel.
    #[serde(rename = "olu")]
    owner: OpenlinkChannelUser,

    /// Unknown.
    vt: i32,

    /// Link type.
    #[serde(rename = "lt")]
    link_type: OpenlinkType,

    /// Description.
    #[serde(rename = "desc")]
    description: String,

    /// Unknown.
    pc: String,

    /// Unknown.
    pa: bool,

    /// Channel activation status.
    /// `true` if this channel is activated.
    #[serde(rename = "ac")]
    activated: bool,

    /// Channel searchable status.
    /// `true` if this channel is searchable.
    #[serde(rename = "sc")]
    searchable: bool,

    /// Link privilege mask.
    #[serde(rename = "pv")]
    privilege: i32,

    /// Unknown.
    ex: bool,

    // /// Unknown.
    // omt: unknown,
    /// Link creation time.
    #[serde(rename = "ca")]
    created_at: i32,

    /// Open channel cover.
    #[serde(rename = "oc")]
    cover: Option<OpenChannelCover>,

    /// Open profile cover.
    #[serde(rename = "op")]
    op: Option<OpenProfileCover>,

    #[serde(flatten)]
    extra: Option<OpenlinkExtra>,
}

/// Open channel cover.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OpenChannelCover {
    /// Open link type.
    #[serde(rename = "t")]
    link_type: OpenlinkType,

    #[serde(rename = "co")]
    option: OpenChannelCoverOption,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OpenChannelCoverOption {
    /// Channel description.
    #[serde(rename = "desc")]
    description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OpenProfileCover {
    /// Profile description.
    #[serde(rename = "desc")]
    description: String,

    tags: Option<Vec<String>>,
}

/// Openlink extra information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OpenlinkExtra {
    /// Max user limit. (for channel)
    #[serde(rename = "ml")]
    max_user: Option<i32>,

    /// Max direct limit. (for profile)
    #[serde(rename = "dcl")]
    max_direct: Option<i32>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(i32)]
pub enum OpenChannelPermission {
    Owner = 1,
    None = 2,
    Manager = 4,
    Bot = 8,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(i32)]
pub enum OpenlinkType {
    Profile = 1,
    Channel = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(i32)]
pub enum OpenProfileType {
    Main = 1,
    AnonymousKakao = 2,
    AnonymousKakao2 = 4,
    Unknown = 8,
    OpenProfile = 16,
}
