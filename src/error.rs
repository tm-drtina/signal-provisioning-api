#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SignalProtocolError(#[from] libsignal_protocol::SignalProtocolError),
    #[error(transparent)]
    SignalCryptoError(#[from] signal_crypto::Error),
    #[error(transparent)]
    SignalCryptoDecryptionError(#[from] signal_crypto::DecryptionError),
    #[error(transparent)]
    UuidParsingError(#[from] uuid::Error),
}

impl From<prost::DecodeError> for Error {
    fn from(err: prost::DecodeError) -> Self {
        err.into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
