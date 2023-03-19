use crate::SignalCryptoError;
use libsignal_protocol::SignalProtocolError;
use prost::DecodeError;

#[derive(Debug)]
pub enum Error {
    SignalProtocolError(SignalProtocolError),
    SignalCryptoError(SignalCryptoError),
    UuidParsingError(uuid::Error),
}

impl From<SignalCryptoError> for Error {
    fn from(err: SignalCryptoError) -> Self {
        Self::SignalCryptoError(err)
    }
}

impl From<SignalProtocolError> for Error {
    fn from(err: SignalProtocolError) -> Self {
        Self::SignalProtocolError(err)
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        err.into()
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Self::UuidParsingError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
