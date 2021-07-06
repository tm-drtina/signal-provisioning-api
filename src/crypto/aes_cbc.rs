//
// Copyright 2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::error::Result;
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use libsignal_protocol::SignalProtocolError;

pub struct Aes256Cbc(Cbc<Aes256, Pkcs7>);

impl Aes256Cbc {
    pub fn new(key: &[u8], iv: &[u8]) -> Result<Self> {
        match Cbc::<Aes256, Pkcs7>::new_from_slices(key, iv) {
            Ok(mode) => Ok(Self(mode)),
            Err(block_modes::InvalidKeyIvLength) => Err(
                SignalProtocolError::InvalidCipherCryptographicParameters(key.len(), iv.len())
                    .into(),
            ),
        }
    }

    pub fn decrypt(self, ctext: &[u8]) -> Result<Vec<u8>> {
        if ctext.is_empty() || ctext.len() % 16 != 0 {
            return Err(SignalProtocolError::InvalidCiphertext.into());
        }

        self.0
            .decrypt_vec(ctext)
            .map_err(|_| SignalProtocolError::InvalidCiphertext.into())
    }
}
