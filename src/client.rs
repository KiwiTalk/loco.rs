use serde::de::DeserializeOwned;
use tokio::{net::TcpStream, sync::{mpsc, oneshot}};
use tokio_native_tls::TlsStream;

use crate::{Result, api::oauth::OAuthCredential, config::ClientConfig, types::{DataStatus, LocoResponse, Method, chat::LoginList}};

#[derive(Clone)]
pub struct Context<Msg> {
    pub message: Msg,
    tx: mpsc::Sender<(Method, Option<oneshot::Sender<LocoResponse>>)>,
}

impl<T> Context<T> {
    pub async fn spawn(&self, request: impl Request) -> Result<()> {
        self.tx.send((request.into(), None)).await?;
        Ok(())
    }

    pub async fn send<Req, Res>(&self, request: Req) -> Result<std::result::Result<Res, DataStatus>>
    where
        Req: Request<Response = Res>,
        Res: DeserializeOwned,
    {
        let (res_tx, res_rx) = oneshot::channel();
        self.tx.send((request.into(), Some(res_tx))).await?;
        let response = res_rx.await?;
        if response.success {
            Ok(Ok(bson::from_document(response.extra)?))
        } else {
            Ok(Err(response.status))
        }
    }
}

pub trait Request: Into<Method> {
    type Response;
}

pub struct LocoClient<Config> {
    config: Config,
    credential: OAuthCredential,
    last_login_revision: i32,
}

impl<Config> LocoClient<Config>
where
    Config: ClientConfig,
{
    async fn run(self) -> Result<()> {
        

        Ok(())
    }
}

async fn create_socket(
    host: &str,
    port: u16,
    // keep_alive: bool,
) -> Result<TlsStream<TcpStream>>{
    let connector = tokio_native_tls::native_tls::TlsConnector::new().unwrap();
    let connector = tokio_native_tls::TlsConnector::from(connector);
    let stream = TcpStream::connect((host, port)).await?;
    let tls_stream = connector.connect(host, stream).await?;
    Ok(tls_stream)
}
