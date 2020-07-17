use crate::{
    net::LocoCodec,
    packet::{LocoPacket, LocoRequest, LocoResponse},
};
use tokio::sync::mpsc::{error::SendError, unbounded_channel, UnboundedSender};
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    stream::StreamExt,
};
use tokio_util::codec::Framed;

#[derive(Clone)]
pub struct Writer {
    tx: UnboundedSender<LocoPacket<LocoRequest>>,
}

impl Writer {
    pub fn send(
        &self,
        packet: LocoPacket<LocoRequest>,
    ) -> Result<(), SendError<LocoPacket<LocoRequest>>> {
        self.tx.send(packet)
    }
}

pub struct HandlerContext {
    pub writer: Writer,
    pub packet: LocoPacket<LocoResponse>,
}

pub struct Dispatcher {
    handler: Option<UnboundedSender<HandlerContext>>,
}

impl Dispatcher {
    pub fn handler<H>(mut self, handler: H) -> Self
    where
        H: FnMut(HandlerContext) + Send + 'static,
    {
        let (tx, rx) = unbounded_channel();
        tokio::spawn(async move { rx.map(handler) });
        self.handler = Some(tx);
        self
    }

    pub async fn run(&mut self, host: impl ToSocketAddrs) -> Result<(), std::io::Error> {
        use futures::SinkExt;
        use log::error;

        let socket = TcpStream::connect(host).await?;
        let mut framed = Framed::new(socket, LocoCodec);
        let (tx, mut rx) = unbounded_channel();
        let writer = Writer { tx };
        loop {
            tokio::select! {
                read = framed.next() => {
                    match read {
                        Some(Ok(packet)) => {
                            if let Some(handler) = &self.handler {
                                handler.send(HandlerContext {
                                    writer: writer.clone(),
                                    packet,
                                }).ok().expect("Packet receiver has been dropped");
                            }
                        }
                        Some(Err(e)) => error!("Could not read packet: {:?}", e),
                        _ => break
                    }
                }
                Some(write) = rx.recv() => {
                    // TODO: better error handling
                    if let Err(e) = framed.send(write).await {
                        error!("Could not send packet: {:?}", e);
                    }
                }
            }
        }
        Ok(())
    }
}
