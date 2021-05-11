use futures::{SinkExt, StreamExt};
use loco::{
    codec::{EncryptType, KeyEncryptType, LocoClientCodec, LocoPacket, LocoSecureClientCodec},
    config::{BookingConfig, CheckinConfig},
    crypto::LocoCrypto,
    types::{
        request,
        response::{self, LocoResponse, ResponseKind},
        UserId,
    },
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use tokio_util::codec::{Decoder, Framed};

use crate::{Error, Result};

pub type LocoSecureClientStream<S> = Framed<S, LocoSecureClientCodec<LocoCrypto>>;
pub type LocoClientStream<S> = Framed<S, LocoClientCodec>;

async fn create_tls_stream(
    host: &str,
    port: u16,
    // keep_alive: bool,
) -> Result<TlsStream<TcpStream>> {
    let connector = tokio_native_tls::native_tls::TlsConnector::new().unwrap();
    let connector = tokio_native_tls::TlsConnector::from(connector);
    let stream = TcpStream::connect((host, port)).await?;
    let tls_stream = connector.connect(host, stream).await?;
    Ok(tls_stream)
}

pub async fn get_config(config: &impl BookingConfig) -> Result<response::GetConf> {
    let (host, port) = config.booking_host();
    let stream = create_tls_stream(host, port).await?;
    let mut framed = LocoClientCodec::default().framed(stream);
    get_booking_data(&mut framed, config).await
}

pub async fn get_checkin(config: &impl CheckinConfig) -> Result<response::Checkin> {
    let crypto = config.new_crypto();
    let try_checkin = match get_config(config).await {
        Ok(response) => {
            let host = response.ticket_hosts.lsl[0].as_str();
            let port = response.config_wifi.ports[0] as u16;
            let stream = TcpStream::connect((host, port)).await?;
            let framed = LocoSecureClientCodec::new(crypto.clone())
                .wrap(
                    stream,
                    KeyEncryptType::RsaOaepSha1Mgf1Sha1,
                    EncryptType::AesCfb128,
                    &config.public_key(),
                )
                .await?;
            Ok(framed)
        }
        Err(e) => Err(e),
    };
    let mut stream = match try_checkin {
        Ok(stream) => stream,
        Err(_) => {
            let (host, port) = config.checkin_fallback_host();
            let stream = TcpStream::connect((host, port)).await?;
            let framed = LocoSecureClientCodec::new(crypto)
                .wrap(
                    stream,
                    KeyEncryptType::RsaOaepSha1Mgf1Sha1,
                    EncryptType::AesCfb128,
                    &config.public_key(),
                )
                .await?;
            framed
        }
    };

    get_checkin_data(&mut stream, config, None).await
}

async fn get_booking_data<S>(
    stream: &mut LocoClientStream<S>,
    config: &impl BookingConfig,
) -> Result<response::GetConf>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let request = request::GetConf::from_config(config);
    let packet = LocoPacket::from_request(0, request);
    stream.send(packet).await?;
    while let Some(packet) = stream.next().await {
        let packet = packet?;
        if packet.id == 0 {
            match packet.payload {
                LocoResponse::Success { status: _, kind } => {
                    if let ResponseKind::GetConf(get_conf) = *kind {
                        return Ok(get_conf);
                    }
                }
                LocoResponse::Fail { status } => {
                    return Err(Error::RequestFail {
                        status,
                        method: packet.method,
                    })
                }
            }
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "No server response").into())
}

async fn get_checkin_data<S>(
    stream: &mut LocoSecureClientStream<S>,
    config: &impl CheckinConfig,
    user_id: Option<UserId>,
) -> Result<response::Checkin>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let request = request::Checkin::from_config(config).with_user(user_id);
    let packet = LocoPacket::from_request(0, request);
    stream.send(packet).await?;
    while let Some(packet) = stream.next().await {
        let packet = packet?;
        if packet.id == 0 {
            match packet.payload {
                LocoResponse::Success { status: _, kind } => {
                    if let ResponseKind::Checkin(checkin) = *kind {
                        return Ok(checkin);
                    }
                }
                LocoResponse::Fail { status } => {
                    return Err(Error::RequestFail {
                        status,
                        method: packet.method,
                    })
                }
            }
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "No server response").into())
}
