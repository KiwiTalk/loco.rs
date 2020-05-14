use crate::{net::{EncodeError, LocoCodec}, packet::{LocoPacket, LocoResponse, LocoRequest}};
use std::sync::Arc;
use tokio::io::WriteHalf;
use tokio::sync::{Mutex, mpsc::{unbounded_channel, UnboundedSender}};
use tokio::{stream::StreamExt, net::TcpStream};
use tokio_util::codec::{Decoder, FramedWrite};
use futures::SinkExt;

pub struct Writer {
    stream: Mutex<FramedWrite<WriteHalf<TcpStream>, LocoCodec>>,
}

impl Writer {
    pub async fn send(&self, packet: LocoPacket<LocoRequest>) -> Result<(), EncodeError> {
        let mut stream = self.stream.lock().await;
        stream.send(packet).await
    }
}

pub struct HandlerContext {
    writer: Arc<Writer>,
    packet: LocoPacket<LocoResponse>,
}

pub struct Dispatcher {
    handler: Option<UnboundedSender<HandlerContext>>,
}

impl Dispatcher {
    pub fn handler<H>(mut self, handler: H) -> Self
    where
        H: FnMut(HandlerContext) -> () + Send + 'static
    {
        let (tx, rx) = unbounded_channel();
        tokio::spawn(async move {
            rx.map(handler)
        });
        self.handler = Some(tx);
        self
    }

    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        todo!()
    }
}
