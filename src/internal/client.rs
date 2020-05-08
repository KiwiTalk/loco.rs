use std::ops::Deref;
use serde_qs;
use std::future::Future;
use reqwest::{Url, Response, Error};
use crate::internal::{AUTH_USER_AGENT, account, LoginData};
use sha2::Digest;

pub struct Client {
    client: reqwest::Client
}

impl Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        return &self.client;
    }
}

impl Client {
    pub fn new() -> Self {
        return Client {
            client: reqwest::Client::new()
        };
    }

    pub fn request_login(&self, login_data: &LoginData) -> impl Future<Output = Result<Response, Error>> {
        return self.post(account::get_login_url())
            .headers(account::get_auth_header(&login_data.to_xvc_key(AUTH_USER_AGENT)))
            .body(serde_qs::to_string(
                login_data
            ).ok().unwrap())
            .send();
    }
}