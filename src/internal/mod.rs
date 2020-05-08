pub mod key;
pub mod emoticon;
pub mod account;

mod language;
mod attachment;
mod xvc_key;
mod client;
mod login_data;
pub mod agent;

pub use attachment::AttachmentType;
pub use language::Language;
pub use xvc_key::XVCKey;
pub use client::Client;
pub use login_data::LoginData;

/*** macros ***/
#[macro_export]
macro_rules! define {
	($i:ident, $e:expr) => {
	    #[macro_export]
		macro_rules! $i {()=>{$e}}
	}
}

#[macro_export]
macro_rules! define_host {
    ($name:ident, $host:literal) => {
        pub mod $name {
            #![allow(dead_code)]
            use reqwest::Url;
            pub const HOST: &str = $host;
            pub const URL: &str = concat!("https://", $host);
            pub fn url() -> Url {
                Url::parse(URL).ok().unwrap()
            }
        }
    };
}


/*** defines ***/
define!{version, "3.1.1"}
define!{internal_app_subversion, 2441}
define!{os_version, "10.0"}
define!{language, "ko"}
//define!{internal_protocol, "https"}
//hosts
define_host!(internal, "sb-talk.kakao.com");

/*** constants ***/
// Maps to InternalAppVersion
pub const APP_VERSION: &str = concat!(version!(), ".", internal_app_subversion!());
// Maps to InternalAppSubVersion
pub const INTERNAL_APP_SUBVERSION: u16 = internal_app_subversion!();
pub const OS_VERSION: &str = os_version!();
pub const LANGUAGE: &str = language!();
pub const AUTH_USER_AGENT: &str = concat!("KT/", version!(), " Wd/", os_version!(), " ", language!());
pub const AUTH_HEADER_AGENT: &str = concat!(agent!(), "/", version!(), "/", language!());
// LOCO entry
pub const LOCO_ENTRY: &str = "booking-loco.kakao.com";
pub const LOCO_ENTRY_PORT: u16 = 443;