use crate::internal::{account, agent::Os, LoginData, DeviceRegisterData, AUTH_USER_AGENT};
use reqwest::{Error, Response};
use std::ops::Deref;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Event {
    Login,
    Disconnected,
    Message,
    MessageRead,
    UserJoin,
    UserLeft,
    JoinChannel,
    LeftChannel,
}

pub struct Client {
    client: reqwest::Client,
    emitter: parallel_event_emitter::ParallelEventEmitter<Event>,
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
            client: Default::default(),
            emitter: Default::default(),
            agent,
        };
    }

    pub async fn request_login(&mut self, login_data: &LoginData) -> Result<Response, Error> {
        let result: Result<Response, Error> = self.post(account::get_login_url(&self.agent))
            .headers(account::get_auth_header(
                &self.agent,
                &login_data.to_xvc_key(AUTH_USER_AGENT),
            ))
            .form(login_data)
            .send()
            .await;

        self.emitter.emit(Event::Login);

        result
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
