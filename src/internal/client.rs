use std::ops::Deref;
use serde_qs;
use std::future::Future;
use reqwest::{Response, Error};
use crate::internal::{agent::Os, AUTH_USER_AGENT, account, LoginData, DeviceRegisterData};
use sha2::Digest;
use std::borrow::Borrow;

pub struct Client {
    client: reqwest::Client,
    agent: Os,
}

impl Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        return &self.client;
    }
}

impl Client {
    pub fn new(agent: Os) -> Self {
        return Client {
            client: reqwest::Client::new(),
            agent,
        };
    }

    pub fn request_login(&self, login_data: &LoginData) -> impl Future<Output = Result<Response, Error>> {
        return self.post(account::get_login_url(self.agent.borrow()))
            .headers(account::get_auth_header(self.agent.borrow(), &login_data.to_xvc_key(AUTH_USER_AGENT)))
            .body(serde_qs::to_string(
                login_data
            ).ok().unwrap())
            .send();
    }

    pub fn request_passcode(&self, login_data: &LoginData) -> impl Future<Output = Result<Response, Error>> {
        return self.post(account::get_request_passcode_url(self.agent.borrow()))
            .headers(account::get_auth_header(self.agent.borrow(), &login_data.to_xvc_key(AUTH_USER_AGENT)))
            .body(serde_qs::to_string(
                login_data
            ).ok().unwrap())
            .send();
    }

    pub fn register_device(&self, device_register_data: &DeviceRegisterData) -> impl Future<Output = Result<Response, Error>> {
        return self.post(account::get_register_device_url(self.agent.borrow()))
            .headers(account::get_auth_header(self.agent.borrow(), &device_register_data.to_xvc_key(AUTH_USER_AGENT)))
            .body(serde_qs::to_string(
                device_register_data
            ).ok().unwrap())
            .send();
    }
}