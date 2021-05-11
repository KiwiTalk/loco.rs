pub struct OAuthCredential {
    pub device_uuid: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub struct OAuthInfo {
    pub token_type: String,
    pub credential: OAuthCredential,
    pub expires_in: u64,
}

pub trait CredentialProvider {
    fn credential(&self) -> OAuthCredential;
}
