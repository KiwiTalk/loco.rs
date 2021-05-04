use bytes::{Buf, BufMut, BytesMut};
use openssl::{
    pkey::{HasPrivate, HasPublic},
    rsa::Rsa,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::{Decoder, Encoder};

use crate::{codec::RsaAesCrypto, Error, Result};

use super::{try_decode_command_data, try_decode_command_header, try_encode_command, LocoCommand};

struct CryptoHeader {
    size: usize,
    iv: [u8; IV_LEN],
}

const CRYPTO_HEADER_LEN: usize = 20;
const IV_LEN: usize = 16;

pub struct LocoSecureCodec<Crypto> {
    crypto_provider: Crypto,
    current_crypto_header: Option<CryptoHeader>,
}

impl<Crypto> LocoSecureCodec<Crypto> {
    pub fn new(crypto_provider: Crypto) -> Self {
        Self {
            crypto_provider,
            current_crypto_header: None,
        }
    }
}

impl<Crypto> Encoder<LocoCommand> for LocoSecureCodec<Crypto>
where
    Crypto: RsaAesCrypto,
{
    type Error = Error;

    fn encode(&mut self, item: LocoCommand, dst: &mut bytes::BytesMut) -> Result<()> {
        let iv = Crypto::random_iv();
        let encrypted = {
            let mut raw = BytesMut::new();
            try_encode_command(&mut raw, item)?;
            self.crypto_provider.encrypt_aes(dbg!(&raw), &iv)?
        };

        dst.reserve(CRYPTO_HEADER_LEN + encrypted.len());
        dst.put_u32_le((encrypted.len() + IV_LEN) as u32);
        dst.put(&iv[..]);
        dst.put(&encrypted[..]);
        Ok(())
    }
}

impl<Crypto> Decoder for LocoSecureCodec<Crypto>
where
    Crypto: RsaAesCrypto,
{
    type Item = LocoCommand;

    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        let current_crypto_header = match self.current_crypto_header.take() {
            header @ Some(_) => header,
            None => try_decode_crypto_header(src)?,
        };

        if let Some(crypto_header) = current_crypto_header {
            if src.len() >= crypto_header.size {
                let decrypted = self
                    .crypto_provider
                    .decrypt_aes(&src[..crypto_header.size], &crypto_header.iv)?;
                src.advance(crypto_header.size);
                let mut inner_src = BytesMut::from(&decrypted[..]);
                let header = try_decode_command_header(dbg!(&mut inner_src))?
                    .expect("No header in the encrypted packet");
                try_decode_command_data(header, &mut inner_src).map(Some)
            } else {
                src.reserve(crypto_header.size);
                self.current_crypto_header = Some(crypto_header);
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

fn try_decode_crypto_header(src: &mut BytesMut) -> Result<Option<CryptoHeader>> {
    if src.len() < CRYPTO_HEADER_LEN {
        src.reserve(CRYPTO_HEADER_LEN);
        Ok(None)
    } else {
        let size = src.get_u32_le() as usize - IV_LEN;
        let mut iv = [0u8; IV_LEN];
        iv.copy_from_slice(&src[..IV_LEN]);
        src.advance(16);
        Ok(Some(CryptoHeader { size, iv }))
    }
}

#[repr(u32)]
pub enum EncryptType {
    AesCfb128 = 2,
}

#[repr(u32)]
pub enum KeyEncryptType {
    RsaOaepSha1Mgf1Sha1 = 12,
}

pub struct LocoHandshake {
    pub key_encrypt_type: u32,
    pub encrypt_type: u32,
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

    let mut buffer = BytesMut::with_capacity(HANDSHAKE_HEADER_LEN + encrypted_key.len());
    buffer.put_u32_le(encrypted_key.len() as u32);
    buffer.put_u32_le(key_encrypt_type as u32);
    buffer.put_u32_le(encrypt_type as u32);
    buffer.put(&encrypted_key[..]);

    socket.write_buf(&mut buffer).await?;
    Ok(buffer.len())
}

pub async fn recv_handshake(
    socket: &mut (impl AsyncReadExt + Unpin),
    crypto: &mut impl RsaAesCrypto,
    key: &Rsa<impl HasPrivate>,
) -> Result<LocoHandshake> {
    let mut buffer = BytesMut::new();
    buffer.resize(HANDSHAKE_HEADER_LEN, 0);
    socket.read_exact(&mut buffer[..]).await?;
    let encrypted_key_len = buffer.get_u32_le() as usize;
    let key_encrypt_type = buffer.get_u32_le();
    let encrypt_type = buffer.get_u32_le();
    let mut encrypted_key = vec![0u8; encrypted_key_len];
    socket.read_exact(&mut encrypted_key).await?;
    crypto.apply_encrypted_key(&encrypted_key, key)?;
    Ok(LocoHandshake {
        key_encrypt_type,
        encrypt_type,
    })
}

#[cfg(test)]
mod test {
    use futures::{SinkExt, StreamExt};
    use tokio::{
        join,
        net::{TcpListener, TcpStream},
    };
    use tokio_util::codec::Framed;

    use crate::{
        codec::LocoCrypto,
        types::{
            chat::{ChatLog, MessagePart},
            Message, Method,
        },
    };

    use super::*;

    #[tokio::test]
    async fn secure_send_recv_should_be_redundant() {
        let command = LocoCommand {
            id: 0,
            status: 0,
            method: Method::Message(Message {
                status: 0,
                message: MessagePart {
                    chat_id: 0,
                    link_id: 0,
                    log_id: 0,
                    chat_log: ChatLog {
                        log_id: 0,
                        chat_id: 0,
                        chat_type: 0,
                        sender_id: 0,
                        message: "".into(),
                        sent_at: 0,
                        attachment: "".into(),
                        msg_id: 0,
                        prev_log_id: 0,
                        supplement: "".into(),
                        referer: 0,
                    },
                    sent_without_seen: false,
                    sender_nickname: None,
                    notification_read: None,
                },
            }),
        };

        let key_server = Rsa::generate(2048).unwrap();
        let key_client = key_server.clone();

        let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let mut client = TcpStream::connect(server.local_addr().unwrap())
            .await
            .unwrap();

        let server_task = async move {
            let mut crypto = LocoCrypto::new().unwrap();
            let (mut remote, _addr) = server.accept().await.unwrap();
            recv_handshake(&mut remote, &mut crypto, &key_server)
                .await
                .unwrap();
            let mut remote = Framed::new(remote, LocoSecureCodec::new(crypto));
            remote.next().await.unwrap().unwrap()
        };

        let client_task = async move {
            let crypto = LocoCrypto::new().unwrap();
            send_handshake(
                &mut client,
                &crypto,
                KeyEncryptType::RsaOaepSha1Mgf1Sha1,
                EncryptType::AesCfb128,
                &key_client,
            )
            .await
            .unwrap();
            let mut client = Framed::new(client, LocoSecureCodec::new(crypto));

            client.send(command.clone()).await.unwrap();
            command
        };

        let (server_res, client_res) = join!(server_task, client_task);
        assert_eq!(server_res, client_res);
    }
}
