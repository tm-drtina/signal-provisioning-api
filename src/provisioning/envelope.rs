//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::crypto::aes_cbc::Aes256Cbc;
use crate::error::{Error, Result};

use std::convert::TryFrom;

use libsignal_protocol::{PrivateKey, PublicKey, SignalProtocolError, HKDF};
use prost::Message;
use signal_crypto::CryptographicMac;
use subtle::ConstantTimeEq;

pub struct ProvisionEnvelope {
    public_key: PublicKey,
    body: Box<[u8]>,
}

impl ProvisionEnvelope {
    const MAC_LENGTH: usize = 32;
    const IV_LENGTH: usize = 16;

    #[inline]
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    #[inline]
    pub fn body_with_headers(&self) -> &[u8] {
        &self.body[..self.body.len() - Self::MAC_LENGTH]
    }

    #[inline]
    pub fn body(&self) -> &[u8] {
        &self.body[Self::IV_LENGTH + 1..self.body.len() - Self::MAC_LENGTH]
    }

    #[inline]
    pub fn iv(&self) -> &[u8] {
        &self.body[1..Self::IV_LENGTH + 1]
    }

    #[inline]
    pub fn mac(&self) -> &[u8] {
        &self.body[self.body.len() - Self::MAC_LENGTH..]
    }

    pub fn decrypt(&self, ephemeral_private_key: PrivateKey) -> Result<Vec<u8>> {
        let ec_res = ephemeral_private_key.calculate_agreement(self.public_key())?;
        let hkdf = HKDF::new(3)?;
        let secrets =
            hkdf.derive_secrets(&ec_res, "TextSecure Provisioning Message".as_bytes(), 64)?;
        let (cipher_key, mac_key) = secrets.split_at(32);

        let mac_valid = self.verify_mac(mac_key)?;

        if !mac_valid {
            return Err(SignalProtocolError::InvalidCiphertext.into());
        }

        Aes256Cbc::new(cipher_key, self.iv())?.decrypt(self.body())
    }

    pub fn verify_mac(&self, mac_key: &[u8]) -> Result<bool> {
        let our_mac = Self::compute_mac(mac_key, self.body_with_headers())?;
        let their_mac = self.mac();
        let result: bool = our_mac.ct_eq(their_mac).into();
        Ok(result)
    }

    fn compute_mac(mac_key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
        if mac_key.len() != 32 {
            return Err(SignalProtocolError::InvalidMacKeyLength(mac_key.len()).into());
        }
        let mut hmac = CryptographicMac::new("HMACSha256", mac_key)?;
        hmac.update(message)?;
        Ok(hmac.finalize()?)
    }
}

impl TryFrom<&[u8]> for ProvisionEnvelope {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let proto_structure = crate::proto::device_messages::ProvisionEnvelope::decode(value)?;

        let body = proto_structure
            .body
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?
            .into_boxed_slice();
        let public_key = proto_structure
            .public_key
            .ok_or(SignalProtocolError::InvalidProtobufEncoding)?;
        let public_key = PublicKey::deserialize(public_key.as_slice())?;

        Ok(ProvisionEnvelope { public_key, body })
    }
}
