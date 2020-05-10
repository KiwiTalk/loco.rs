use crate::internal::{LoginData, account, Client, AUTH_USER_AGENT, DeviceRegisterData};
use reqwest::{Error, Response};

pub struct Certification {
}

impl Certification {
    pub async fn login(client: &Client, login_data: &LoginData) -> Result<Response, Error> {
        client.post(account::get_login_url(&client.agent))
            .headers(account::get_auth_header(
                &client.agent,
                &login_data.to_xvc_key(AUTH_USER_AGENT),
            ))
            .form(login_data)
            .send()
            .await
    }

    pub async fn passcode(client: &Client, login_data: &LoginData) -> Result<Response, Error> {
        client.post(account::get_request_passcode_url(&client.agent))
            .headers(account::get_auth_header(
                &client.agent,
                &login_data.to_xvc_key(AUTH_USER_AGENT),
            ))
            .form(login_data)
            .send()
            .await
    }

    pub async fn device(client: &Client, device_register_data: &DeviceRegisterData) -> Result<Response, Error> {
        client.post(account::get_register_device_url(&client.agent))
            .headers(account::get_auth_header(&client.agent, &device_register_data.to_xvc_key(AUTH_USER_AGENT)))
            .form(device_register_data)
            .send()
            .await
    }
}
