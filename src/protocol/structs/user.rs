/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Deserialize, Serialize};

use super::open_link::OpenUser;

/// Minimal user info for chatroom display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayUserInfo {
    /// User id
    #[serde(rename = "userId")]
    pub user_id: i64,

    /// User nickname
    #[serde(rename = "nickName")]
    pub nickname: String,

    /// Profile image URL. None if profile image is default.
    #[serde(rename = "pi")]
    pub profile_image_url: Option<String>,

    /// Country Iso, does not present on openchat.
    #[serde(rename = "countryIso", skip_serializing_if = "Option::is_none")]
    pub country_iso: Option<String>,
}

/// User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: i64,

    #[serde(rename = "nickName")]
    pub nickname: String,

    #[serde(rename = "countryIso")]
    pub country_iso: String,

    #[serde(rename = "profileImageUrl")]
    pub profile_image_url: Option<String>,

    #[serde(rename = "fullProfileImageUrl")]
    pub full_profile_image_url: Option<String>,

    #[serde(rename = "OriginalProfileImageUrl")]
    pub original_profile_image_url: Option<String>,

    /// See UserType for types.
    #[serde(rename = "type")]
    pub user_type: i32,

    #[serde(rename = "accountId")]
    pub account_id: i64,

    #[serde(rename = "linkedServices")]
    pub linked_services: String,

    #[serde(rename = "statusMessage")]
    pub status_message: String,

    pub suspended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserVariant {
    Normal(User),
    Open(OpenUser),
}

/// User types. Don't confuse with OpenMemberType.
#[repr(i32)]
pub enum UserType {
    Unknown = -999999,
    NotFriend = -100,
    Deactivated = 9,
    Friend = 100,
    Openchat = 1000,
}
