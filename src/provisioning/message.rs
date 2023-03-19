//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::error::{Error, Result};

use std::convert::TryFrom;

use libsignal_protocol::{IdentityKey, IdentityKeyPair, PrivateKey, SignalProtocolError};
use prost::Message;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProvisionMessage {
    aci: Uuid,
    aci_identity_key_pair: IdentityKeyPair,
    pni: Uuid,
    pni_identity_key_pair: IdentityKeyPair,
    number: String,
    provisioning_code: String,
    user_agent: Option<String>,
    read_receipts: Option<bool>,
}

impl ProvisionMessage {
    #[inline]
    pub fn aci_identity_key_pair(&self) -> &IdentityKeyPair {
        &self.aci_identity_key_pair
    }

    #[inline]
    pub fn aci(&self) -> &Uuid {
        &self.aci
    }

    #[inline]
    pub fn pni_identity_key_pair(&self) -> &IdentityKeyPair {
        &self.pni_identity_key_pair
    }

    #[inline]
    pub fn pni(&self) -> &Uuid {
        &self.pni
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

        let aci = proto_structure
            .aci
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?
            .parse()?;
        let aci_private_key = proto_structure
            .aci_identity_key_private
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;
        let aci_private_key = PrivateKey::deserialize(aci_private_key.as_slice())?;
        let aci_identity_key_pair = IdentityKeyPair::new(
            IdentityKey::new(aci_private_key.public_key()?),
            aci_private_key,
        );

        let pni = proto_structure
            .pni
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?
            .parse()?;
        let pni_private_key = proto_structure
            .pni_identity_key_private
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;
        let pni_private_key = PrivateKey::deserialize(pni_private_key.as_slice())?;
        let pni_identity_key_pair = IdentityKeyPair::new(
            IdentityKey::new(pni_private_key.public_key()?),
            pni_private_key,
        );

        let number = proto_structure
            .number
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;
        let provisioning_code = proto_structure
            .provisioning_code
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;

        Ok(ProvisionMessage {
            aci,
            aci_identity_key_pair,
            pni,
            pni_identity_key_pair,
            number,
            provisioning_code,
            user_agent: proto_structure.user_agent,
            read_receipts: proto_structure.read_receipts,
        })
    }
}
