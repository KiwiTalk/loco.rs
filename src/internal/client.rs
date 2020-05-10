use crate::internal::{account, agent::Os, request};
use reqwest::{Error, Response};
use std::ops::Deref;

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
}
