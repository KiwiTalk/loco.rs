pub trait WebApiConfig {
    fn agent(&self) -> &str;
    fn device_model(&self) -> &str;
    fn language(&self) -> &str;
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
