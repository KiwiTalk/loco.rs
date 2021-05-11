#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Openssl(openssl::error::ErrorStack),
    Reqwest(reqwest::Error),
    BsonSerialize(Box<bson::ser::Error>),
    BsonDeserialize(Box<bson::de::Error>),
    Bincode(bincode::Error),
    InvalidCryptoKey,
    UnsupportedPacketType,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(e: openssl::error::ErrorStack) -> Self {
        Self::Openssl(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<bson::ser::Error> for Error {
    fn from(e: bson::ser::Error) -> Self {
        Self::BsonSerialize(Box::new(e))
    }
}

impl From<bson::de::Error> for Error {
    fn from(e: bson::de::Error) -> Self {
        Self::BsonDeserialize(Box::new(e))
    }
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Self::Bincode(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
