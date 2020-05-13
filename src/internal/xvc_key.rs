use sha2::{Digest, Sha512};
use std::ops::Deref;

pub struct XVCKey {
    key: String,
}

impl XVCKey {
    pub fn new(header: &str, email: &str, device_uuid: &str) -> Self {
        XVCKey {
            key: hex::encode(Sha512::digest(
                format!("HEATH|{}|DEMIAN|{}|{}", header, email, device_uuid).as_bytes(),
            )),
        }
    }
}

impl Deref for XVCKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.key
    }
}
