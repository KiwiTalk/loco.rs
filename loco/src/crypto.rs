use openssl::{
    pkey::{HasPrivate, HasPublic},
    rsa::Rsa,
};

use crate::{Error, Result};

pub trait RsaAesCrypto {
    fn encrypt_key(&self, key: &Rsa<impl HasPublic>) -> Result<Vec<u8>>;
    fn apply_encrypted_key(
        &mut self,
        encrypted_aes_key: &[u8],
        key: &Rsa<impl HasPrivate>,
    ) -> Result<()>;

    fn encrypt_aes(&self, data: &[u8], iv: &[u8; 16]) -> Result<Vec<u8>>;
    fn decrypt_aes(&self, data: &[u8], iv: &[u8; 16]) -> Result<Vec<u8>>;

    fn random_iv() -> [u8; 16] {
        let mut iv = [0_u8; 16];
        openssl::rand::rand_bytes(&mut iv).expect("This shouldn't happen!");
        iv
    }
}

#[derive(Clone)]
pub struct LocoCrypto {
    aes_key: [u8; 16],
}

impl LocoCrypto {
    pub fn new() -> Result<Self> {
        let mut aes_key = [0u8; 16];
        openssl::rand::rand_bytes(&mut aes_key)?;
        Ok(Self::new_with_key(aes_key))
    }

    pub fn new_with_key(aes_key: [u8; 16]) -> Self {
        Self { aes_key }
    }
}

impl RsaAesCrypto for LocoCrypto {
    fn encrypt_key(&self, key: &Rsa<impl HasPublic>) -> Result<Vec<u8>> {
        let mut out = vec![0u8; 256];
        key.public_encrypt(&self.aes_key, &mut out, openssl::rsa::Padding::PKCS1_OAEP)?;
        Ok(out)
    }

    fn apply_encrypted_key(
        &mut self,
        encrypted_aes_key: &[u8],
        key: &Rsa<impl HasPrivate>,
    ) -> Result<()> {
        let mut aes_key = vec![0_u8; 256];
        let size = key.private_decrypt(
            &encrypted_aes_key,
            &mut aes_key,
            openssl::rsa::Padding::PKCS1_OAEP,
        )?;
        if size != 16 {
            Err(Error::InvalidCryptoKey)
        } else {
            self.aes_key.copy_from_slice(&aes_key[..16]);
            Ok(())
        }
    }

    fn encrypt_aes(&self, data: &[u8], iv: &[u8; 16]) -> Result<Vec<u8>> {
        let cipher = openssl::symm::Cipher::aes_128_cfb128();
        let encrypted = openssl::symm::encrypt(cipher, &self.aes_key, Some(iv), data)?;
        Ok(encrypted)
    }

    fn decrypt_aes(&self, data: &[u8], iv: &[u8; 16]) -> Result<Vec<u8>> {
        let cipher = openssl::symm::Cipher::aes_128_cfb128();
        let decrypted = openssl::symm::decrypt(cipher, &self.aes_key, Some(iv), data)?;
        Ok(decrypted)
    }
}
