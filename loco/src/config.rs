use crate::{crypto::LocoCrypto, types::NetType};

pub trait AgentConfig {
    fn agent(&self) -> &str;
    fn device_model(&self) -> &str;
}

pub trait LanguageConfig {
    fn language(&self) -> &str;
}

pub trait WebApiConfig: AgentConfig + LanguageConfig {
    fn os_version(&self) -> &str;
    fn version(&self) -> &str;

    fn api_path(&self, api: impl AsRef<str>) -> String;
    fn user_agent(&self) -> String;
}

pub trait XvcProvider {
    fn to_full_xvc_key(
        &self,
        device_uuid: impl AsRef<str>,
        user_agent: impl AsRef<str>,
        email: impl AsRef<str>,
    ) -> String;
}

pub trait BookingConfig: AgentConfig {
    fn booking_host(&self) -> (&str, u16);
    fn mccmnc(&self) -> &str;
}

pub trait CheckinConfig: BookingConfig + LanguageConfig {
    fn checkin_fallback_host(&self) -> (&str, u16);
    fn use_sub_device(&self) -> bool;
    fn app_version(&self) -> &str;
    fn country_iso(&self) -> &str;
    fn net_type(&self) -> NetType;
    fn new_crypto(&self) -> LocoCrypto;
    fn public_key(&self) -> &Rsa<Public>;
}

use openssl::{pkey::Public, rsa::Rsa};
pub use CheckinConfig as SessionConfig;

pub type DeviceType = i32;
pub trait ClientConfig: SessionConfig + WebApiConfig {
    fn device_type(&self) -> DeviceType;
}

pub trait OAuthLoginConfig: WebApiConfig {
    fn login_token_seeds(&self) -> (&str, &str);
}

pub(crate) trait WebApiRequest {
    fn with_api_a_header(self, web_api: &impl WebApiConfig) -> Self;
    fn with_api_base_header(self, web_api: &impl WebApiConfig) -> Self;
}

impl WebApiRequest for reqwest::RequestBuilder {
    fn with_api_a_header(self, web_api: &impl WebApiConfig) -> Self {
        self.header(
            "A",
            &format!(
                "{}/{}/{}",
                web_api.agent(),
                web_api.version(),
                web_api.language()
            ),
        )
    }

    fn with_api_base_header(self, web_api: &impl WebApiConfig) -> Self {
        self.header(reqwest::header::ACCEPT, "*/*")
            .header(reqwest::header::ACCEPT_LANGUAGE, web_api.language())
    }
}
