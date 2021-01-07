use crate::internal::StatusCode;
use serde::{Deserialize, Serialize};

// Maps to LoginAccessDataStruct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAccessData {
    pub status: Option<StatusCode>,
    pub story_url: Option<String>,
    pub user_id: Option<i32>,
    pub country_iso: Option<String>,
    pub country_code: Option<String>,
    pub account_id: Option<i32>,
    pub server_time: Option<i32>,
    pub reset_user_data: Option<bool>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub auto_login_id: Option<String>,
    pub display_id: Option<String>,
    pub main_device: Option<String>,
    pub main_device_app_version: Option<String>,
}
