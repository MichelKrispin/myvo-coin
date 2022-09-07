use crate::crypto::hash;

use serde;

pub const OUTPUT_LENGTH: usize = 8 + hash::HASH_LENGTH;

/// Represents the output of a transactions
/// with the amount of coins that belongs
/// to this output and a hash of the public key
/// that identifies the owner uniquely.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// The amount of coins that belong to this output.
    amount: u64,

    /// The hash of the owner of this output and its coins.
    owner: hash::Hash,
}

impl Output {
    /// Creates a new output with the given amount and
    /// the new owners hashed public key.
    pub fn create(amount: u64, owner: hash::Hash) -> Self {
        Self { amount, owner }
    }

    /// Gets the owners hash.
    pub fn get_owner_hash(&self) -> &hash::Hash {
        &self.owner
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    pub fn as_bytes(&self) -> [u8; OUTPUT_LENGTH] {
        let v1 = &self.amount.to_be_bytes();
        let v2 = self.owner.as_bytes();
        let whole: Vec<u8> = v1.iter().chain(v2.iter()).map(|v| *v).collect();
        let whole: [u8; OUTPUT_LENGTH] = whole.try_into().unwrap();
        whole
    }
}

use std::fmt;

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Output] {} coins to <{}>", self.amount, self.owner)
    }
}
