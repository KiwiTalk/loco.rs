use std::ops::Deref;

use reqwest::{Error, Response};

use crate::internal::{account, agent::Os, AUTH_USER_AGENT, DeviceRegisterData, LoginData};

pub struct TokenClient {
    client: reqwest::Client,
    agent: Os,
}

impl Deref for TokenClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        return &self.client;
    }
}

impl TokenClient {
    pub fn new(agent: Os) -> Self {
        return TokenClient {
            client: Default::default(),
            agent,
        };
    }

    pub async fn request_login(&self, login_data: &LoginData) -> Result<Response, Error> {
        self.post(account::get_login_url(&self.agent))
            .headers(account::get_auth_header(
                &self.agent,
                &login_data.to_xvc_key(AUTH_USER_AGENT),
            ))
            .form(login_data)
            .send()
            .await
    }

    pub async fn request_passcode(&self, login_data: &LoginData) -> Result<Response, Error> {
        self.post(account::get_request_passcode_url(&self.agent))
            .headers(account::get_auth_header(
                &self.agent,
                &login_data.to_xvc_key(AUTH_USER_AGENT),
            ))
            .form(login_data)
            .send()
            .await
    }

    pub async fn register_device(&self, device_register_data: &DeviceRegisterData) -> Result<Response, Error> {
        self.post(account::get_register_device_url(&self.agent))
            .headers(account::get_auth_header(&self.agent, &device_register_data.to_xvc_key(AUTH_USER_AGENT)))
            .form(device_register_data)
            .send()
            .await
    }
}
