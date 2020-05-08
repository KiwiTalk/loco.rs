use crate::internal::XVCKey;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceRegisterData {
    email: String,
    password: String,
    device_uuid: String,
    device_name: String,
    os_version: String,
    permanent: bool,
    passcode: u16,
}

impl DeviceRegisterData {
    pub fn new(email: String, password: String, device_uuid: String, device_name: String, os_version: String, permanent: bool, passcode: u16) -> DeviceRegisterData {
        return DeviceRegisterData {
            email,
            password,
            device_uuid,
            device_name,
            os_version,
            permanent,
            passcode,
        };
    }

    pub fn to_xvc_key(&self, auth_user_agent: &str) -> XVCKey {
        XVCKey::new(auth_user_agent, self.email.as_ref(), self.device_uuid.as_ref())
    }
}