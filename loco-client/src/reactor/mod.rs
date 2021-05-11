use std::collections::HashMap;

use futures::{
    channel::{mpsc, oneshot},
    future::Either,
    SinkExt, Stream, StreamExt,
};
use loco::{
    codec::{self, EncryptType, KeyEncryptType, LocoPacket, LocoSecureDecoder, LocoSecureEncoder},
    config::ClientConfig,
    types::{
        request::LocoRequest,
        response::{LocoResponse, ResponseKind},
    },
};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{Error, Result};

mod entrance;

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<(LocoRequest, Option<oneshot::Sender<LocoResponse>>)>,
}

impl Sender {
    pub async fn spawn(&mut self, request: impl Into<LocoRequest>) -> Result<()> {
        self.tx.send((request.into(), None)).await?;
        Ok(())
    }

    pub async fn send(&mut self, request: impl Into<LocoRequest>) -> Result<ResponseKind> {
        let (res_tx, res_rx) = oneshot::channel();
        let request: LocoRequest = request.into();
        let method = request.to_string();
        self.tx.send((request, Some(res_tx))).await?;
        let response = res_rx.await?;
        match response {
            LocoResponse::Success { kind, .. } => Ok(*kind),
            LocoResponse::Fail { status } => Err(Error::RequestFail { status, method }),
        }
    }
}

pub struct Reactor<Config> {
    config: Config,
    sender: Sender,
    receiver: mpsc::Receiver<(LocoRequest, Option<oneshot::Sender<LocoResponse>>)>,
    packet_tx: Option<mpsc::Sender<(Sender, LocoPacket<LocoResponse>)>>,
}

impl<Config> Reactor<Config> {
    pub fn new(config: Config) -> Self {
        let (sender, receiver) = mpsc::channel(16);
        Self {
            config,
            sender: Sender { tx: sender },
            receiver,
            packet_tx: None,
        }
    }

    pub fn packets(&mut self) -> impl Stream<Item = (Sender, LocoPacket<LocoResponse>)> {
        let (packet_tx, packet_rx) = mpsc::channel(16);
        self.packet_tx = Some(packet_tx);
        packet_rx
    }

    pub fn sender(&self) -> &Sender {
        &self.sender
    }
}

impl<Config> Reactor<Config>
where
    Config: ClientConfig + Send + 'static,
{
    pub async fn spawn(self) -> Result<()> {
        let checkin = entrance::get_checkin(&self.config).await?;
        let mut socket = TcpStream::connect((checkin.host.as_str(), checkin.port as u16)).await?;
        let crypto = self.config.new_crypto();
        codec::send_handshake(
            &mut socket,
            &crypto,
            KeyEncryptType::RsaOaepSha1Mgf1Sha1,
            EncryptType::AesCfb128,
            &self.config.public_key(),
        )
        .await?;
        let (rx, tx) = socket.into_split();
        let reader = FramedRead::new(rx, LocoSecureDecoder::new(crypto.clone()));
        let mut writer = FramedWrite::new(tx, LocoSecureEncoder::new(crypto));
        let (mut req_tx, req_rx) = mpsc::channel(16);

        let mut packet_tx = self.packet_tx;
        let sender = self.sender;
        let read_task = async move {
            let mut notifier_registry =
                HashMap::<i32, Option<oneshot::Sender<LocoResponse>>>::new();

            let reader_stream = reader.map(Either::Left);
            let register_stream = req_rx.map(Either::Right);

            let mut stream = futures::stream::select(reader_stream, register_stream);
            while let Some(input) = stream.next().await {
                match input {
                    Either::Left(Ok(packet)) => {
                        if let Some(Some(notifier)) = notifier_registry.remove(&packet.id) {
                            notifier
                                .send(packet.payload)
                                .expect("Response notification failed");
                            continue;
                        }
                        if let Some(tx) = &mut packet_tx {
                            tx.send((sender.clone(), packet)).await?;
                        }
                    }
                    Either::Left(Err(e)) => return Err(e.into()),
                    Either::Right((id, notifier)) => {
                        notifier_registry.insert(id, notifier);
                    }
                }
            }

            Ok(())
        };

        let mut receiver = self.receiver;
        let write_task = async move {
            let mut packet_id = 0;
            while let Some((request, maybe_notifier)) = receiver.next().await {
                let packet = LocoPacket::from_request(packet_id, request);
                writer.send(packet).await?;
                req_tx.send((packet_id, maybe_notifier)).await?;
                packet_id += 1;
            }

            Ok(())
        };

        let (try_read, try_write) = futures::future::join(read_task, write_task).await;
        try_read.and(try_write)
    }
}
