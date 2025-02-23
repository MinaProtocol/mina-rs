// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account token symbol

use ark_ff::FromBytes;
use derive_more::{From, Into};
use proof_systems::{mina_hasher::Fp, ChunkedROInput, ToChunkedROInput};

/// FIXME: Need to learn exactly what this is..
#[derive(Clone, Debug, Default, From, Into)]
pub struct TokenSymbol([u8; 32]);

impl TokenSymbol {
    /// FIXME: Need to learn exactly what this is..
    pub const fn max_length() -> usize {
        6
    }

    /// FIXME: Need to learn exactly what this is..
    pub const fn num_bits() -> usize {
        8 * Self::max_length()
    }
}

impl ToChunkedROInput for TokenSymbol {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        let mut bytes = [0; 32];
        // FIXME: This might not be correct
        bytes[..Self::max_length()].copy_from_slice(&self.0[..Self::max_length()]);
        let f = Fp::read(&bytes[..]).unwrap();
        ChunkedROInput::new().append_packed(f, TokenSymbol::num_bits() as u32)
    }
}
