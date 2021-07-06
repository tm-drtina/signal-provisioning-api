//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::error::{Error, Result};

use std::convert::TryFrom;

use libsignal_protocol::{IdentityKey, IdentityKeyPair, PrivateKey, SignalProtocolError};
use prost::Message;

#[derive(Clone)]
pub struct ProvisionMessage {
    identity_key_pair: IdentityKeyPair,
    number: String,
    provisioning_code: String,
    uuid: Option<String>,
    user_agent: Option<String>,
    read_receipts: Option<bool>,
}

impl ProvisionMessage {
    #[inline]
    pub fn identity_key_pair(&self) -> &IdentityKeyPair {
        &self.identity_key_pair
    }

    #[inline]
    pub fn number(&self) -> &String {
        &self.number
    }

    #[inline]
    pub fn provisioning_code(&self) -> &String {
        &self.provisioning_code
    }

    #[inline]
    pub fn uuid(&self) -> &Option<String> {
        &self.uuid
    }

    #[inline]
    pub fn user_agent(&self) -> &Option<String> {
        &self.user_agent
    }

    #[inline]
    pub fn read_receipts(&self) -> &Option<bool> {
        &self.read_receipts
    }
}

impl TryFrom<&[u8]> for ProvisionMessage {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let proto_structure = crate::proto::device_messages::ProvisionMessage::decode(value)?;

        let private_key = proto_structure
            .identity_key_private
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;
        let private_key = PrivateKey::deserialize(private_key.as_slice())?;
        let public_key = private_key.public_key()?;
        let identity_key_pair = IdentityKeyPair::new(IdentityKey::new(public_key), private_key);

        let number = proto_structure
            .number
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;
        let provisioning_code = proto_structure
            .provisioning_code
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;
        let uuid = proto_structure.uuid.map(|x| x.to_lowercase());

        Ok(ProvisionMessage {
            identity_key_pair,
            number,
            provisioning_code,
            uuid,
            user_agent: proto_structure.user_agent,
            read_receipts: proto_structure.read_receipts,
        })
    }
}
