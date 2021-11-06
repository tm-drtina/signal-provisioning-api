use std::convert::TryFrom;
use std::time::SystemTime;

use rand::rngs::OsRng;

use bytes::Bytes;
use prost::Message as ProstMessage;

use libsignal_protocol::{KeyPair, SignalProtocolError};

use crate::error::Result;
use crate::proto::sub_protocol::{
    web_socket_message::Type::{Request, Response},
    WebSocketMessage, WebSocketRequestMessage, WebSocketResponseMessage,
};

use super::{ProvisionEnvelope, ProvisionMessage, ProvisioningUuid};

pub enum ProvisioningState {
    Init,
    UuidReceived(ProvisioningUuid),
    Provisioned(ProvisionMessage),
}

pub struct ProvisioningSocket {
    pub ephemeral_key_pair: KeyPair,
    pub state: ProvisioningState,
}

impl ProvisioningSocket {
    pub fn new() -> Self {
        Self {
            ephemeral_key_pair: KeyPair::generate(&mut OsRng),
            state: ProvisioningState::Init,
        }
    }

    pub fn serialize<T: ProstMessage>(msg: T) -> Vec<u8> {
        msg.encode_to_vec()
    }

    pub fn acknowledge(request_id: u64) -> WebSocketMessage {
        let response = WebSocketResponseMessage {
            id: Some(request_id),
            status: Some(200),
            message: Some("OK".to_string()),
            headers: vec!["Content-Length:0".to_string()],
            body: None,
        };
        WebSocketMessage {
            r#type: Some(Response as i32),
            request: None,
            response: Some(response),
        }
    }

    fn request(request: WebSocketRequestMessage) -> WebSocketMessage {
        WebSocketMessage {
            r#type: Some(Request as i32),
            request: Some(request),
            response: None,
        }
    }

    pub fn hb_request() -> WebSocketMessage {
        let request = WebSocketRequestMessage {
            verb: Some("GET".to_string()),
            path: Some("/v1/keepalive/provisioning".to_string()),
            body: None,
            headers: vec![],
            id: Some(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("now cannot be earlier then Unix epoch")
                    .as_millis() as u64,
            ),
        };
        Self::request(request)
    }

    pub fn process_message(&mut self, bytes: Vec<u8>) -> Result<Option<u64>> {
        let bytes = Bytes::from(bytes);
        let proto_structure = WebSocketMessage::decode(bytes)?;
        match proto_structure {
            WebSocketMessage {
                r#type: Some(1),
                request: Some(request),
                response: None,
            } => {
                let request_id = request
                    .id
                    .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;

                if request.verb == Some("PUT".to_string())
                    && request.path == Some("/v1/address".to_string())
                {
                    let body = request
                        .body
                        .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;

                    let uuid = ProvisioningUuid::try_from(body.as_slice())?;

                    self.state = ProvisioningState::UuidReceived(uuid);
                    Ok(Some(request_id))
                } else if request.verb == Some("PUT".to_string())
                    && request.path == Some("/v1/message".to_string())
                {
                    let body = request
                        .body
                        .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;

                    let envelope = ProvisionEnvelope::try_from(body.as_slice())?;
                    let plaintext = envelope.decrypt(self.ephemeral_key_pair.private_key)?;
                    let message = ProvisionMessage::try_from(plaintext.as_slice())?;

                    self.state = ProvisioningState::Provisioned(message);
                    Ok(Some(request_id))
                } else {
                    Err(SignalProtocolError::InvalidProtobufEncoding.into())
                }
            }
            WebSocketMessage {
                r#type: Some(2),
                request: None,
                response: Some(_response),
            } => {
                // Ignoring responses from server.
                // There should be only HB responses.
                Ok(None)
            }
            _ => Err(SignalProtocolError::InvalidProtobufEncoding.into()),
        }
    }
}

impl Default for ProvisioningSocket {
    fn default() -> Self {
        Self::new()
    }
}
