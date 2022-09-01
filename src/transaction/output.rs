use crate::crypto::hash;

/// Represents the output of a transactions
/// with the amount of coins that belongs
/// to this output and a hash of the public key
/// that identifies the owner uniquely.
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
}

use std::fmt;

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Output] {} coins of {}", self.amount, self.owner)
    }
}
