pub mod key;
pub mod emoticon;

mod language;
mod attachment;

pub use attachment::AttachmentType;
pub use language::Language;

/** macros **/
macro_rules! define {
	($i:ident, $e:expr) => {
		macro_rules! $i {()=>{$e}}
	}
}

#[macro_export]
macro_rules! define_host {
    ($name:ident, $host:literal) => {
        pub mod $name {
            use reqwest::Url;
            pub const URL: &str = concat!("https://", $host);
            pub fn url() -> Url {
                Url::parse(URL).ok().unwrap()
            }
        }
    };
}


/** defines **/
define!{agent, "win32"}
define!{version, "3.1.1"}
define!{internal_app_subversion, 2441}
define!{os_version, "10.0"}
define!{language, "ko"}
//define!{internal_protocol, "https"}
define!{account_path, "account"}
//hosts
define_host!(account, "ac-sb-talk.kakao.com");
define_host!(internal, "sb-talk.kakao.com");

/** constants **/
// Maps to InternalAppVersion
pub const APP_VERSION: &str = concat!(version!(), ".", internal_app_subversion!());
// Maps to InternalAppSubVersion
pub const INTERNAL_APP_SUBVERSION: u16 = internal_app_subversion!();
pub const OS_VERSION: &str = os_version!();
pub const LANGUAGE: &str = language!();
pub const AUTH_USER_AGENT: &str = concat!("KT/", version!(), " Wd/", os_version!(), " ", language!());
pub const AUTH_HEADER_AGENT: &str = concat!(version!(), "/", os_version!(), "/", language!());
// LOCO entry
pub const LOCO_ENTRY: &str = "booking-loco.kakao.com";
pub const LOCO_ENTRY_PORT: u16 = 443;


pub fn get_account_internal_path(path: &str) -> String {
    format!(concat!(agent!(), "/", account_path!(), "/{}"), path)
}

// TODO: Account, LogonAccount, getInternalURL, getAccountInternalUrl, getEmoticonHeader



// TODO
