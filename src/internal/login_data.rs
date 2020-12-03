use data_encoding::BASE64;
use serde::{Deserialize, Serialize};

use crate::internal::XVCKey;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct LoginData {
    email: String,
    password: String,
    device_uuid: String,
    device_name: String,
    os_version: String,
    permanent: bool,
    forced: bool,
}

impl LoginData {
    pub fn new(
        email: String,
        password: String,
        device_uuid: &str,
        device_name: String,
        os_version: String,
        permanent: bool,
        forced: bool,
    ) -> Self {
        LoginData {
            email,
            password,
            device_uuid: BASE64.encode(device_uuid.as_bytes()),
            device_name,
            os_version,
            permanent,
            forced,
        }
    }

    pub fn to_xvc_key(&self, auth_user_agent: &str) -> XVCKey {
        XVCKey::new(
            auth_user_agent,
            self.email.as_ref(),
            self.device_uuid.as_ref(),
        )
    }
}
