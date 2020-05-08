// Maps to LoginAccessDataStruct
#[derive(Debug, Clone)]
pub struct LoginAccessData {
    status: i32,
    story_url: String,
    user_id: i32,
    country_iso: String,
    country_code: String,
    account_id: i32,
    server_time: i32,
    reset_user_data: bool,
    access_token: String,
    refresh_token: String,
    token_type: String,
    auto_login_id: String,
    display_id: String,
    main_device: String,
    main_device_app_version: String,
}
