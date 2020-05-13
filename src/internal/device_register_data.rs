use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::internal::LoginData;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceRegisterData {
    #[serde(flatten)]
    login_data: LoginData,
    pub passcode: String,
}

impl Deref for DeviceRegisterData {
    type Target = LoginData;

    fn deref(&self) -> &Self::Target {
        return &self.login_data;
    }
}

impl DeviceRegisterData {
    pub fn new(login_data: LoginData, passcode: String) -> DeviceRegisterData {
        return DeviceRegisterData {
            login_data,
            passcode
        };
    }
}