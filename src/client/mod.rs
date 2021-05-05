use serde::de::DeserializeOwned;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
};
use tokio_util::codec::{Decoder, FramedRead, FramedWrite};

use crate::{
    api::oauth::OAuthCredential,
    codec::{LocoCrypto, LocoSecureCodec},
    config::ClientConfig,
    types::{DataStatus, LocoResponse, Method},
    Error, Result,
};

use self::entrance::get_checkin;

mod entrance;

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<(Method, Option<oneshot::Sender<LocoResponse>>)>,
}

impl Sender {
    pub async fn spawn(&self, request: impl Request) -> Result<()> {
        self.tx.send((request.into(), None)).await?;
        Ok(())
    }

    pub async fn send<Req, Res>(&self, request: Req) -> Result<Res>
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

pub struct LocoClient<Config> {
    config: Config,
    credential: OAuthCredential,
    last_login_revision: i32,
    sender: Sender,
    recceiver: mpsc::Receiver<(Method, Option<oneshot::Sender<LocoResponse>>)>,
}

impl<Config> LocoClient<Config>
where
    Config: ClientConfig,
{
    pub async fn run(self) -> Result<()> {
        let checkin = get_checkin(&self.config).await?;
        let mut socket = TcpStream::connect((checkin.host.as_str(), checkin.port as u16)).await?;
        let crypto = self.config.new_crypto();
        let (rx, tx) = socket.split();
        let reader = FramedRead::new(rx, LocoSecureCodec::new(crypto.clone()));
        let writer = FramedWrite::new(tx, LocoSecureCodec::new(crypto));

        Ok(())
    }
}
