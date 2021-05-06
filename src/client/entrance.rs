use futures::{Sink, SinkExt, Stream, StreamExt};
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use tokio_util::codec::Decoder;

use crate::{
    codec::{LocoCodec, LocoPacket, LocoSecureCodec},
    config::{BookingConfig, CheckinConfig},
    types::{
        booking::{GetConf, GetConfRes},
        chat::UserId,
        checkin::{Checkin, CheckinRes},
        LocoData,
    },
    Error, Result,
};

use super::Response;

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

pub async fn get_config(config: &impl BookingConfig) -> Result<GetConfRes> {
    let (host, port) = config.booking_host();
    let stream = create_tls_stream(host, port).await?;
    let mut framed = LocoCodec::new().framed(stream);
    get_booking_data(&mut framed, config).await
}

pub async fn get_checkin(config: &impl CheckinConfig) -> Result<CheckinRes> {
    let crypto = config.new_crypto();
    let try_checkin = match get_config(config).await {
        Ok(config) => {
            let host = config.ticket_hosts.lsl[0].as_str();
            let port = config.config_wifi.ports[0] as u16;
            let stream = TcpStream::connect((host, port)).await?;
            Ok(LocoSecureCodec::new(crypto.clone()).framed(stream))
        }
        Err(e @ Error::FailedRequest(_)) => return Err(e),
        Err(e) => Err(e),
    };
    let mut stream = match try_checkin {
        Ok(stream) => stream,
        Err(_) => {
            let (host, port) = config.checkin_fallback_host();
            let stream = TcpStream::connect((host, port)).await?;
            LocoSecureCodec::new(crypto).framed(stream)
        }
    };

    get_checkin_data(&mut stream, config, None).await
}

async fn get_booking_data<S, E>(stream: &mut S, config: &impl BookingConfig) -> Result<GetConfRes>
where
    S: Stream<Item = std::result::Result<LocoPacket, E>> + Sink<LocoPacket, Error = E> + Unpin,
    Error: From<E>,
{
    let request = GetConf::from_config(config);
    let packet = LocoPacket::from_method(0, 0, request);
    stream.send(packet).await?;
    while let Some(packet) = stream.next().await {
        if let LocoData::Response(response) = packet?.data {
            if GetConfRes::is_success(response.status) {
                let get_conf_res = bson::from_document(response.extra)?;
                return Ok(get_conf_res);
            } else {
                return Err(Error::FailedRequest(response.status));
            }
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "No server response").into())
}

async fn get_checkin_data<S, E>(
    stream: &mut S,
    config: &impl CheckinConfig,
    user_id: Option<UserId>,
) -> Result<CheckinRes>
where
    S: Stream<Item = std::result::Result<LocoPacket, E>> + Sink<LocoPacket, Error = E> + Unpin,
    Error: From<E>,
{
    let request = Checkin::from_config(config).with_user(user_id);
    let packet = LocoPacket::from_method(0, 0, request);
    stream.send(packet).await?;
    while let Some(packet) = stream.next().await {
        if let LocoData::Response(response) = packet?.data {
            if CheckinRes::is_success(response.status) {
                let checkin_res = bson::from_document(response.extra)?;
                return Ok(checkin_res);
            } else {
                return Err(Error::FailedRequest(response.status));
            }
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "No server response").into())
}
