//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::error::{Error, Result};

use std::convert::TryFrom;

use base64::{Engine as _, engine::general_purpose};
use libsignal_protocol::{PublicKey, SignalProtocolError};
use prost::Message;

pub struct ProvisioningUuid {
    uuid: String,
}

impl ProvisioningUuid {
    #[inline]
    pub fn uuid(&self) -> &String {
        &self.uuid
    }

    pub fn provisioning_url(&self, ephemeral_pubkey: PublicKey) -> String {
        let public_key_base64 = general_purpose::STANDARD.encode(ephemeral_pubkey.serialize());
        // We need to urlencode, but base64 has limited alphabet, so replace is enough
        let public_key_base64 = public_key_base64.replace('+', "%2B").replace('/', "%2F");
        format!(
            "tsdevice:/?uuid={}&pub_key={}",
            self.uuid, public_key_base64
        )
    }
}

impl TryFrom<&[u8]> for ProvisioningUuid {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let proto_structure = crate::proto::device_messages::ProvisioningUuid::decode(value)?;

        let uuid = proto_structure
            .uuid
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;

        Ok(ProvisioningUuid { uuid })
    }
}
