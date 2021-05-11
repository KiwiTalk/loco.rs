use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{ChatId, LinkId, OpenChannelPermission, OpenProfileType};

pub type UserId = i64;

/// Normal channel member.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NormalMember {
    /// User ID.
    user_id: UserId,

    /// Account ID.
    account_id: i32,

    /// User nickname.
    #[serde(rename = "nickName")]
    nickname: String,

    /// User's country in ISO 3166 format.
    country_iso: String,

    /// Profile image URL.
    profile_image_url: String,

    /// Full profile image URL.
    full_profile_image_url: String,

    /// Original profile image URL.
    original_profile_image_url: String,

    /// Profile status message.
    status_message: String,

    /// Linked kakao services.
    linked_services: String,

    /// Account suspension state.
    /// `true` if the account is suspended.
    suspended: bool,

    /// User type. (uncertain)
    #[serde(rename = "ut")]
    user_type: i32,
}

/// Open channel member.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OpenMember {
    /// User ID.
    #[serde(rename = "userId")]
    user_id: UserId,

    /// User type.
    #[serde(rename = "type")]
    user_type: UserType,

    /// Account suspension state.
    /// `true` if the account is suspended.
    suspended: bool,

    /// User nickname.
    #[serde(rename = "nickName")]
    nickname: String,

    /// Profile image URL.
    #[serde(rename = "pi")]
    profile_image_url: String,

    /// Full profile image URL.
    #[serde(rename = "fpi")]
    full_profile_image_url: String,

    /// Original profile image URL.
    #[serde(rename = "opi")]
    original_profile_image_url: String,

    /// Open token.
    #[serde(rename = "opt")]
    open_token: i32,

    /// Profile link ID. Only presents if user is using open profile.
    pli: Option<LinkId>,

    /// Open channel user permission.
    #[serde(rename = "mt")]
    user_permission: OpenChannelPermission,
}

/// Kicked openlink member.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct KickedOpenlinkMember {
    /// User ID.
    #[serde(rename = "userId")]
    user_id: UserId,

    /// User nickname.
    #[serde(rename = "nickName")]
    nickname: String,

    /// Profile image URL.
    #[serde(rename = "pi")]
    profile_image_url: String,

    /// Original channel ID.
    #[serde(rename = "c")]
    kicked_from: ChatId,

    /// Unknown.
    dc: bool,
}

/// Openlink channel user.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OpenlinkChannelUser {
    /// User ID.
    #[serde(rename = "userId")]
    user_id: UserId,

    /// Openlink profile type.
    #[serde(rename = "ptp")]
    profile_type: OpenProfileType,

    /// Open chat user permission.
    #[serde(rename = "lmt")]
    lmt: OpenChannelPermission,

    /// User nickname.
    #[serde(rename = "nn")]
    nickname: String,

    /// Profile image URL.
    #[serde(rename = "pi")]
    profile_image_url: String,

    /// Full profile image URL.
    #[serde(rename = "fpi")]
    full_profile_image_url: String,

    /// Original profile image URL.
    #[serde(rename = "opi")]
    original_profile_image_url: String,

    /// Open token.
    #[serde(rename = "opt")]
    open_token: i32,

    /// Profile link ID.
    pli: LinkId,

    /// Special link privilege mask.
    #[serde(rename = "pv")]
    privilege: i32,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(i32)]
pub enum UserType {
    Undefined = -999999,
    NotFriend = -100,
    Deactivated = 9,
    Friend = 100,
    OpenProfile = 1000,
}
