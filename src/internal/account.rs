use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Url};
use crate::{define_host, define};
use crate::internal::{os::Os, XVCKey, AUTH_HEADER_AGENT, AUTH_USER_AGENT, LANGUAGE};

define!{account_path, "account"}

define_host!(account, "ac-sb-talk.kakao.com");

pub fn get_internal_path(agent: Os, path: &str) -> String {
    format!(concat!("{}/", account_path!(), "/{}"), agent.as_str(), path)
}

pub fn get_auth_header(xvc_key: &XVCKey) -> HeaderMap<HeaderValue> {
    let mut header_map = HeaderMap::new();
    header_map.append(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    header_map.append(header::HOST, HeaderValue::from_static(account::HOST));
    header_map.append("A", HeaderValue::from_static(AUTH_HEADER_AGENT));
    header_map.append("X-VC", HeaderValue::from_str(&xvc_key[0..16]).ok().unwrap());
    header_map.append(header::USER_AGENT, HeaderValue::from_static(AUTH_USER_AGENT));
    header_map.append(header::ACCEPT_LANGUAGE, HeaderValue::from_static(LANGUAGE));
    return header_map;
}

pub fn get_login_url(os: Os) -> Url {
    let mut url: Url = account::url();
    url.set_path(
        get_internal_path(os, paths::LOGIN).as_ref()
    );
    url
}

pub mod paths {
    pub const LOGIN: &str = "login.json";
    pub const REGISTER_DEVICE: &str = "register_device.json";
    pub const REQUEST_PASSCODE: &str = "request_passcode.json";
    pub const LOGIN_TOKEN: &str = "login_token.json";
    pub const REQUEST_VERIFY_EMAIL: &str = "request_verify_email.json";
    pub const RENEW_TOKEN: &str = "renew_token.json";
    pub const CHANGE_UUID: &str = "change_uuid.json";
    pub const CAN_CHANGE_UUID: &str = "can_change_uuid.json";
}