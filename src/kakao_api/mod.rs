pub mod path;

use crate::{LoginData, XVCKey};
use reqwest::{Url, Response, Error};
use path::account_path;
use std::ops::Deref;
use serde_qs;
use std::future::Future;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header;

macro_rules! define {
	($i:ident, $e:expr) => {
		macro_rules! $i {()=>{$e}}
	}
}

define!{agent, "win32"}
define!{version, "3.1.1"}
define!{internal_app_subversion, 2441}
define!{os_version, "10.0"}
define!{language, "ko"}
define!{internal_protocol, "https"}
define!{account_internal_host, "ac-sb-talk.kakao.com"}
define!{account_internal_url, Url::parse(format!("{}://{}", INTERNAL_PROTOCOL, ACCOUNT_INTERNAL_HOST).as_ref()).ok().unwrap()}

define!{account_path, "account"}

pub const INTERNAL_APP_VERSION: &str = concat!(version!(), ".", internal_app_subversion!());
pub const INTERNAL_APP_SUBVERSION: u16 = internal_app_subversion!();
pub const OS_VERSION: &str = os_version!();
pub const LANGUAGE: &str = language!();
pub const AUTH_USER_AGENT: &str = concat!("KT/", version!(), " Wd/", os_version!(), " ", language!());
pub const AUTH_HEADER_AGENT: &str = concat!(version!(), "/", os_version!(), "/", language!());
pub const INTERNAL_PROTOCOL: &str = internal_protocol!();
pub const ACCOUNT_INTERNAL_HOST: &str = account_internal_host!();

pub fn get_account_internal_path(path: &str) -> String {
	return format!(concat!(agent!(), "/", account_path!(), "/{}"), path);
}

pub fn get_auth_header(xvc_key: &XVCKey) -> HeaderMap<HeaderValue> {
	let mut header_map = HeaderMap::new();
	header_map.append(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
	header_map.append(header::HOST, HeaderValue::from_static(ACCOUNT_INTERNAL_HOST));
	header_map.append("A", HeaderValue::from_static(AUTH_HEADER_AGENT));
	header_map.append("X-VC", HeaderValue::from_str(&xvc_key[0..16]).ok().unwrap());
	header_map.append(header::USER_AGENT, HeaderValue::from_static(AUTH_USER_AGENT));
	header_map.append(header::ACCEPT_LANGUAGE, HeaderValue::from_static(LANGUAGE));
	return header_map;
}

pub struct Client {
	client: reqwest::Client
}

impl Deref for Client {
	type Target = reqwest::Client;

	fn deref(&self) -> &Self::Target {
		return &self.client;
	}
}

impl Client {
	pub fn new() -> Self {
		return Client {
			client: reqwest::Client::new()
		};
	}

	pub fn request_login(&self, login_data: &LoginData) -> impl Future<Output = Result<Response, Error>> {
		let mut url: Url = account_internal_url!();
		url.set_path(
			get_account_internal_path(account_path::LOGIN).as_ref()
		);
		return self.post(url)
			.headers(get_auth_header(&login_data.to_xvc_key(AUTH_USER_AGENT)))
			.body(serde_qs::to_string(
				login_data
			).ok().unwrap())
			.send();
	}
}



//TODO
