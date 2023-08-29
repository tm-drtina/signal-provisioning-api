#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Signal protocol error")]
    SignalProtocolError(#[from] libsignal_protocol::SignalProtocolError),
    #[error("Error from signal crypto module")]
    SignalCryptoError(#[from] signal_crypto::Error),
    #[error("Decryption failed")]
    SignalCryptoDecryptionError(#[from] signal_crypto::DecryptionError),
    #[error("Invalid UUID")]
    UuidParsingError(#[from] uuid::Error),
}

impl From<prost::DecodeError> for Error {
    fn from(err: prost::DecodeError) -> Self {
        err.into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
