use crate::internal::{LoginData, account, Client, AUTH_USER_AGENT, DeviceRegisterData};
use reqwest::{Error, Response};

pub struct Certification {
    client: Client,
}

impl Certification {
    pub fn new(client: Client) -> Self {
        return Certification {
            client
        };
    }

    pub async fn login(&self, login_data: &LoginData) -> Result<Response, Error> {
        self.client.post(account::get_login_url(&self.client.agent))
            .headers(account::get_auth_header(
                &self.client.agent,
                &login_data.to_xvc_key(AUTH_USER_AGENT),
            ))
            .form(login_data)
            .send()
            .await
    }

    pub async fn passcode(&self, login_data: &LoginData) -> Result<Response, Error> {
        self.client.post(account::get_request_passcode_url(&self.client.agent))
            .headers(account::get_auth_header(
                &self.client.agent,
                &login_data.to_xvc_key(AUTH_USER_AGENT),
            ))
            .form(login_data)
            .send()
            .await
    }

    pub async fn device(&self, device_register_data: &DeviceRegisterData) -> Result<Response, Error> {
        self.client.post(account::get_register_device_url(&self.client.agent))
            .headers(account::get_auth_header(&self.client.agent, &device_register_data.to_xvc_key(AUTH_USER_AGENT)))
            .form(device_register_data)
            .send()
            .await
    }
}
