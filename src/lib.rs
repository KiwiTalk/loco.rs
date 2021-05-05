pub mod api;
mod client;
pub mod codec;
pub mod config;
pub mod types;
pub use client::*;
use types::DataStatus;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    InvalidPacketDataType(u8),
    UnsupportedMethod(String),
    BsonDeserialize(bson::de::Error),
    BsonSerialize(bson::ser::Error),
    Openssl(openssl::error::ErrorStack),
    InvalidCryptoKey,
    Channel,
    Tls(tokio_native_tls::native_tls::Error),
    FailedRequest(DataStatus),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<bson::de::Error> for Error {
    fn from(error: bson::de::Error) -> Self {
        Self::BsonDeserialize(error)
    }
}

impl From<bson::ser::Error> for Error {
    fn from(error: bson::ser::Error) -> Self {
        Self::BsonSerialize(error)
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(error: openssl::error::ErrorStack) -> Self {
        Self::Openssl(error)
    }
}

impl From<tokio_native_tls::native_tls::Error> for Error {
    fn from(e: tokio_native_tls::native_tls::Error) -> Self {
        Self::Tls(e)
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::Channel
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::Channel
    }
}

pub type Result<T> = std::result::Result<T, Error>;
