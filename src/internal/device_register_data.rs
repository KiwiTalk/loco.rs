use crate::internal::{XVCKey, LoginData};
use serde::{Serialize, Deserialize};
use std::ops::Deref;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceRegisterData {
    #[serde(flatten)]
    login_data: LoginData,
    pub passcode: u16,
}

impl Deref for DeviceRegisterData {
    type Target = LoginData;

    fn deref(&self) -> &Self::Target {
        return &self.login_data;
    }
}

impl DeviceRegisterData {
    pub fn new(login_data: LoginData, passcode: u16) -> DeviceRegisterData {
        return DeviceRegisterData {
            login_data,
            passcode
        };
    }
}