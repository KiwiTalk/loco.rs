use crate::internal::LoginData;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceRegisterData {
    #[serde(flatten)]
    login_data: LoginData,
    pub passcode: String,
}

impl Deref for DeviceRegisterData {
    type Target = LoginData;

    fn deref(&self) -> &Self::Target {
        &self.login_data
    }
}

impl DeviceRegisterData {
    pub fn new(login_data: LoginData, passcode: String) -> DeviceRegisterData {
        DeviceRegisterData {
            login_data,
            passcode,
        }
    }
}
