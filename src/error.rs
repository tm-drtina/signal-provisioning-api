use libsignal_protocol::SignalProtocolError;
use prost::DecodeError;
use signal_crypto::Error as SignalCryptoError;

#[derive(Debug)]
pub enum Error {
	SignalProtocolError(SignalProtocolError),
	SignalCryptoError(SignalCryptoError),
}

impl From<SignalCryptoError> for Error {
	fn from(err: SignalCryptoError) -> Self {
		Error::SignalCryptoError(err)
	}
}

impl From<SignalProtocolError> for Error {
	fn from(err: SignalProtocolError) -> Self {
		Error::SignalProtocolError(err)
	}
}

impl From<DecodeError> for Error {
	fn from(err: DecodeError) -> Self {
		SignalProtocolError::from(err).into()
	}
}

pub type Result<T> = std::result::Result<T, Error>;
