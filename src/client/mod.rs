use std::collections::HashMap;

use futures::{
    channel::{mpsc, oneshot},
    future::{join, Either},
    SinkExt, Stream, StreamExt,
};
use serde::de::DeserializeOwned;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{
    codec::{LocoPacket, LocoSecureCodec},
    config::ClientConfig,
    types::{DataStatus, LocoData, LocoResponse, Method},
    Error, Result,
};

use self::entrance::get_checkin;

mod entrance;

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<(Method, Option<oneshot::Sender<LocoResponse>>)>,
}

impl Sender {
    pub async fn spawn(&mut self, request: impl Request) -> Result<()> {
        self.tx.send((request.into(), None)).await?;
        Ok(())
    }

    pub async fn send<Req, Res>(&mut self, request: Req) -> Result<Res>
    where
        Req: Request<Response = Res>,
        Res: Response,
    {
        let (res_tx, res_rx) = oneshot::channel();
        self.tx.send((request.into(), Some(res_tx))).await?;
        let response = res_rx.await?;
        if Res::is_success(response.status) {
            Ok(bson::from_document(response.extra)?)
        } else {
            Err(Error::FailedRequest(response.status))
        }
    }
}

pub trait Request: Into<Method> {
    type Response;
}

pub trait Response: DeserializeOwned {
    fn is_success(status: DataStatus) -> bool {
        status == LocoResponse::SUCCESS
    }
}

pub struct LocoEventLoop<Config> {
    config: Config,
    sender: Sender,
    receiver: mpsc::Receiver<(Method, Option<oneshot::Sender<LocoResponse>>)>,
    packet_tx: Option<mpsc::Sender<(Sender, LocoPacket)>>,
}

impl<Config> LocoEventLoop<Config> {
    pub fn new(config: Config) -> Self {
        let (sender, receiver) = mpsc::channel(16);
        Self {
            config,
            sender: Sender { tx: sender },
            receiver,
            packet_tx: None,
        }
    }

    pub fn packets(&mut self) -> impl Stream<Item = (Sender, LocoPacket)> {
        let (packet_tx, packet_rx) = mpsc::channel(16);
        self.packet_tx = Some(packet_tx);
        packet_rx
    }

    pub fn sender(&self) -> &Sender {
        &self.sender
    }
}

impl<Config> LocoEventLoop<Config>
where
    Config: ClientConfig + Send + 'static,
{
    pub async fn spawn(self) -> Result<()> {
        let checkin = get_checkin(&self.config).await?;
        let socket = TcpStream::connect((checkin.host.as_str(), checkin.port as u16)).await?;
        let crypto = self.config.new_crypto();
        let (rx, tx) = socket.into_split();
        let reader = FramedRead::new(rx, LocoSecureCodec::new(crypto.clone()));
        let mut writer = FramedWrite::new(tx, LocoSecureCodec::new(crypto));
        let (mut req_tx, req_rx) = mpsc::channel(16);

        let mut packet_tx = self.packet_tx;
        let sender = self.sender;
        let read_task = async move {
            let mut notifier_registry =
                HashMap::<u32, Option<oneshot::Sender<LocoResponse>>>::new();

            let reader_stream = reader.map(Either::Left);
            let register_stream = req_rx.map(Either::Right);

            let mut stream = futures::stream::select(reader_stream, register_stream);
            while let Some(input) = stream.next().await {
                match input {
                    Either::Left(Ok(mut packet)) => {
                        if let LocoData::Response(response) = packet.data {
                            if let Some(Some(notifier)) = notifier_registry.remove(&packet.id) {
                                notifier
                                    .send(response)
                                    .expect("Response notification failed");
                                continue;
                            }
                            packet.data = LocoData::Response(response);
                        }
                        if let Some(tx) = &mut packet_tx {
                            tx.send((sender.clone(), packet)).await?;
                        }
                    }
                    Either::Left(Err(e)) => return Err(e),
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
            while let Some((method, maybe_notifier)) = receiver.next().await {
                let packet = LocoPacket::from_method(packet_id, 0, method);
                writer.send(packet).await?;
                req_tx.send((packet_id, maybe_notifier)).await?;
                packet_id += 1;
            }

            Ok(())
        };

        let (try_read, try_write) = join(read_task, write_task).await;
        try_read.and(try_write)
    }
}
