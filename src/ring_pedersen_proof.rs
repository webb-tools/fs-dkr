/*
    zk-paillier
    Copyright 2018 by Kzen Networks
    zk-paillier is free software: you can redistribute
    it and/or modify it under the terms of the GNU General Public
    License as published by the Free Software Foundation, either
    version 3 of the License, or (at your option) any later version.
    @license GPL-3.0+ <https://github.com/KZen-networks/zk-paillier/blob/master/LICENSE>
*/
use std::iter;
use std::ops::Shl;

use serde::{Deserialize, Serialize};

use curv::arithmetic::traits::*;
use curv::BigInt;
use paillier::{extract_nroot, DecryptionKey, EncryptionKey};
use rayon::prelude::*;

use super::errors::IncorrectProof;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RingPedersenProof {
}

impl RingPedersenProof {
    pub fn proof(dk: &DecryptionKey, salt_str: Option<&'static [u8]>) -> RingPedersenProof {
        
    }

    pub fn verify(&self, ek: &EncryptionKey, salt_str: &[u8]) -> Result<(), IncorrectProof> {

    }
}

#[cfg(test)]
mod tests {
}