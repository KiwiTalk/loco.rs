use crate::config::{WebApiConfig, XvcProvider};
use crate::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};

use crate::config::WebApiRequest;

pub struct LoginClient<WebApi, Xvc> {
    client: Client,
    config: WebApi,
    xvc_provider: Xvc,
    device_uuid: String,
    device_name: String,
}

#[derive(Serialize)]
struct LoginForm<'form> {
    email: &'form str,
    password: &'form str,
    forced: Option<bool>,
    device_uuid: &'form str,
    device_name: &'form str,
    model_name: Option<&'form str>,
}

impl<WebApi, Xvc> LoginClient<WebApi, Xvc>
where
    WebApi: WebApiConfig,
    Xvc: XvcProvider,
{
    pub async fn login(
        &self,
        email: impl AsRef<str>,
        password: impl AsRef<str>,
        forced: Option<bool>,
    ) -> Result<LoginResponse> {
        let api_path = self.config.api_path("login.json");
        let model_name = self.config.device_model();
        let form = LoginForm {
            email: email.as_ref(),
            password: password.as_ref(),
            forced,
            device_uuid: &self.device_uuid,
            device_name: &self.device_name,
            model_name: (!model_name.is_empty()).then(|| model_name),
        };
        let result = self
            .client
            .post(&api_path)
            .with_api_a_header(&self.config)
            .with_api_base_header(&self.config)
            .headers(
                self.headers(email.as_ref())
                    .expect("Invalid web api configuration"),
            )
            .form(&form)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }

    fn headers(
        &self,
        email: &str,
    ) -> std::result::Result<HeaderMap, reqwest::header::InvalidHeaderValue> {
        let mut header_map = HeaderMap::new();
        let user_agent = self.config.user_agent();
        let xvc = self
            .xvc_provider
            .to_full_xvc_key(&self.device_uuid, &user_agent, email);
        header_map.insert(
            reqwest::header::USER_AGENT,
            self.config.user_agent().parse()?,
        );
        header_map.insert("X-VC", xvc.parse()?);
        Ok(header_map)
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub account_id: String,
    pub auto_login_account_id: String,
    pub country_code: String,
    pub country_iso: String,
    pub device_uuid: String,
    pub display_account_id: String,
    pub main_device_agent_name: String,
    pub main_device_app_version: String,
    pub refresh_token: String,
    pub reset_user_data: bool,
    pub server_time: i64,
    pub token_type: String,
    pub user_id: i64,
}
