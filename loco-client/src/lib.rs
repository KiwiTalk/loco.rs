use loco::types::response::DataStatus;

pub mod reactor;

#[derive(Debug)]
pub enum Error {
    Loco(Box<loco::Error>),
    Io(std::io::Error),
    Tls(tokio_native_tls::native_tls::Error),
    ReactorFail,
    RequestFail { status: DataStatus, method: String },
    PacketIdConflict(i32),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<loco::Error> for Error {
    fn from(e: loco::Error) -> Self {
        Self::Loco(Box::new(e))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<tokio_native_tls::native_tls::Error> for Error {
    fn from(e: tokio_native_tls::native_tls::Error) -> Self {
        Self::Tls(e)
    }
}

impl From<futures::channel::oneshot::Canceled> for Error {
    fn from(_: futures::channel::oneshot::Canceled) -> Self {
        Self::ReactorFail
    }
}

impl From<futures::channel::mpsc::SendError> for Error {
    fn from(_: futures::channel::mpsc::SendError) -> Self {
        Self::ReactorFail
    }
}
