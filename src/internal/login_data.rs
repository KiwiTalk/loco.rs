use crate::internal::XVCKey;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct LoginData {
    email: String,
    password: String,
    device_uuid: String,
    os_version: String,
    device_name: String,
    permanent: bool
}

impl LoginData {
    pub fn new(email: String, password: String, device_uuid: String, device_name: String, permanent: bool, os_version: String) -> LoginData {
        return LoginData {
            email,
            password,
            device_uuid,
            os_version,
            device_name,
            permanent
        };
    }

    pub fn to_xvc_key(&self, auth_user_agent: &str) -> XVCKey {
        XVCKey::new(auth_user_agent, self.email.as_ref(), self.device_uuid.as_ref())
    }
}