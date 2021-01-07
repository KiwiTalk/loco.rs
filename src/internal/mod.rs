pub use attachment::AttachmentType;
pub use device_register_data::DeviceRegisterData;
pub use language::Language;
pub use login_access_data::LoginAccessData;
pub use login_data::LoginData;
pub use status_code::StatusCode;
pub use token_client::TokenClient;
pub use xvc_key::XVCKey;

pub mod account;
pub mod emoticon;
pub mod key;

mod attachment;
mod device_register_data;
mod language;
mod login_access_data;
mod login_data;
mod status_code;
mod token_client;
mod xvc_key;

/*** macros ***/
#[macro_export]
macro_rules! define {
    ($i:ident, $e:expr) => {
        #[macro_export]
        macro_rules! $i {
            () => {
                $e
            };
        }
    };
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
                Url::parse(URL).expect(concat!("Illegal url: ", "https://", $host))
            }
        }
    };
}

/*** defines ***/
define! {version, "3.1.1"}
define! {internal_app_subversion, 2441}
define! {os_version, "10.0"}
define! {language, "ko"}
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
pub const AUTH_USER_AGENT: &str =
    concat!("KT/", version!(), " Wd/", os_version!(), " ", language!());
pub const AUTH_HEADER_WITHOUT_AGENT: &str = concat!("/", version!(), "/", language!());
// LOCO entry
pub const LOCO_ENTRY: &str = "booking-loco.kakao.com";
pub const LOCO_ENTRY_PORT: u16 = 443;
