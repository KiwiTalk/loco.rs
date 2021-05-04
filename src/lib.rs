pub mod api;
pub mod codec;
pub mod config;
pub mod types;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    InvalidPacketDataType(u8),
    UnsupportedMethod(String),
    BsonDeserialize(bson::de::Error),
    BsonSerialize(bson::ser::Error),
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

pub type Result<T> = std::result::Result<T, Error>;
