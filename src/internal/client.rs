use crate::internal::{account, agent::Os, request};
use reqwest::{Error, Response};
use std::ops::Deref;

pub struct Request {
    pub certification: request::Certification,
}

impl Request {
    fn new(client: Client) -> Self {
        return Request {
            certification: request::Certification::new(client)
        };
    }
}

pub struct Client {
    pub client: reqwest::Client,
    pub agent: Os,
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
            agent,
        };
    }

    pub fn new_request(self) -> Request {
        return Request::new(self);
    }
}
