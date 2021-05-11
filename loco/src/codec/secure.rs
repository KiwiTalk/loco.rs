use bytes::{Buf, BufMut, BytesMut};
use futures::{AsyncReadExt, AsyncWriteExt};
use openssl::{
    pkey::{HasPrivate, HasPublic},
    rsa::Rsa,
};
use tokio_util::codec::{Decoder, Encoder};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{crypto::RsaAesCrypto, Error, Result};

use super::{LocoDecoder, LocoEncoder, LocoPacket};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct CryptoHeader {
    iv: [u8; IV_LEN],
    size_plus_16: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct CryptoPacket {
    #[serde(flatten)]
    header: CryptoHeader,
    data: Vec<u8>,
}

const CRYPTO_HEADER_LEN: usize = 20;
const IV_LEN: usize = 16;

pub struct LocoSecureEncoder<Crypto> {
    crypto_provider: Crypto,
    inner: LocoEncoder,
}

impl<Crypto> LocoSecureEncoder<Crypto> {
    pub fn new(crypto_provider: Crypto) -> Self {
        Self {
            crypto_provider,
            inner: LocoEncoder,
        }
    }
}

impl<Crypto, Payload> Encoder<LocoPacket<Payload>> for LocoSecureEncoder<Crypto>
where
    Crypto: RsaAesCrypto,
    Payload: Serialize,
{
    type Error = Error;

    fn encode(&mut self, item: LocoPacket<Payload>, dst: &mut bytes::BytesMut) -> Result<()> {
        let iv = Crypto::random_iv();
        let encrypted = {
            let mut raw = BytesMut::new();
            self.inner.encode(item, &mut raw)?;
            self.crypto_provider.encrypt_aes(&raw, &iv)?
        };
        let header = CryptoHeader {
            size_plus_16: encrypted.len() as u32 + 16,
            iv,
        };
        let packet = CryptoPacket {
            header,
            data: encrypted,
        };
        bincode::serialize_into(dst.writer(), &packet)?;
        Ok(())
    }
}

pub struct LocoSecureDecoder<Crypto, Payload> {
    crypto_provider: Crypto,
    prev_crypto_header: Option<CryptoHeader>,
    inner: LocoDecoder<Payload>,
}

impl<Crypto, Payload> Decoder for LocoSecureDecoder<Crypto, Payload>
where
    Crypto: RsaAesCrypto,
    Payload: DeserializeOwned,
{
    type Item = LocoPacket<Payload>;

    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        let header = if let Some(header) = self.prev_crypto_header.take() {
            header
        } else if src.len() < CRYPTO_HEADER_LEN {
            src.reserve(CRYPTO_HEADER_LEN);
            return Ok(None);
        } else {
            bincode::deserialize_from(src.reader())?
        };

        let data_size = header.size_plus_16 as usize - 16;
        if src.len() < data_size {
            src.reserve(data_size);
            self.prev_crypto_header = Some(header);
            Ok(None)
        } else {
            let decrypted = self
                .crypto_provider
                .decrypt_aes(&src[..data_size], &header.iv)?;
            src.advance(decrypted.len());
            let mut inner_src = BytesMut::from(&decrypted[..]);
            let item = self
                .inner
                .decode(&mut inner_src)?
                .expect("Size specified in secure header is not equal to actual size");
            Ok(Some(item))
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(u32)]
pub enum EncryptType {
    AesCfb128 = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(u32)]
pub enum KeyEncryptType {
    RsaOaepSha1Mgf1Sha1 = 12,
}

pub struct LocoHandshake {
    pub key_encrypt_type: KeyEncryptType,
    pub encrypt_type: EncryptType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct RawLocoHandshake {
    pub key_size: u32,
    pub key_encrypt_type: KeyEncryptType,
    pub encrypt_type: EncryptType,
}

const HANDSHAKE_HEADER_LEN: usize = 12;

pub async fn send_handshake(
    socket: &mut (impl AsyncWriteExt + Unpin),
    crypto: &impl RsaAesCrypto,
    key_encrypt_type: KeyEncryptType,
    encrypt_type: EncryptType,
    key: &Rsa<impl HasPublic>,
) -> Result<usize> {
    let encrypted_key = crypto.encrypt_key(key)?;

    let mut buffer = bincode::serialize(&RawLocoHandshake {
        key_size: encrypted_key.len() as u32,
        key_encrypt_type,
        encrypt_type,
    })?;
    buffer.reserve(encrypted_key.len());
    buffer.extend_from_slice(&encrypted_key[..]);

    socket.write_all(&buffer).await?;
    Ok(buffer.len())
}

pub async fn recv_handshake(
    socket: &mut (impl AsyncReadExt + Unpin),
    crypto: &mut impl RsaAesCrypto,
    key: &Rsa<impl HasPrivate>,
) -> Result<LocoHandshake> {
    let mut buffer = [0u8; HANDSHAKE_HEADER_LEN];
    socket.read_exact(&mut buffer).await?;
    let header: RawLocoHandshake = bincode::deserialize(&buffer[..])?;
    let mut encrypted_key = vec![0u8; header.key_size as usize];
    socket.read_exact(&mut encrypted_key).await?;
    crypto.apply_encrypted_key(&encrypted_key, key)?;
    Ok(LocoHandshake {
        key_encrypt_type: header.key_encrypt_type,
        encrypt_type: header.encrypt_type,
    })
}
