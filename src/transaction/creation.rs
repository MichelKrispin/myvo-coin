use super::output;
use crate::crypto::hash;

use serde;

/// The default number of coins that will be created
/// when a new block is added to the chain.
const DEFAULT_CREATION_AMOUNT: u64 = 1;
pub const CREATION_LENGTH: usize = output::OUTPUT_LENGTH + hash::HASH_LENGTH;

/// A creation block that should be included
/// at the top of a block.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Creation {
    /// The new owner of the creation's block new coins.
    output: output::Output,

    /// The hash of this block to make it resemble a
    /// 'normal' transaction.
    id_hash: hash::Hash,
}

impl Creation {
    /// Create a new creation transaction with the new owner.
    ///
    /// # Arguments
    /// new_owner: The new owner of this creation's output.
    pub fn new(new_owner: hash::Hash) -> Self {
        let output = output::Output::create(DEFAULT_CREATION_AMOUNT, new_owner);
        let id_hash = {
            let bytes = output.as_bytes();
            hash::Hash::create(bytes)
        };
        Self { output, id_hash }
    }

    pub fn get_output(&self) -> &output::Output {
        &self.output
    }

    pub fn as_bytes(&self) -> [u8; CREATION_LENGTH] {
        let v1 = self.id_hash.as_bytes();
        let v2 = &self.output.as_bytes();
        let whole: Vec<u8> = v1.iter().chain(v2.iter()).map(|v| *v).collect();
        let whole: [u8; CREATION_LENGTH] = whole.try_into().unwrap();
        whole
    }
}

use std::fmt;

impl fmt::Display for Creation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Creation]:{}\n  {}", self.id_hash, self.output,)
    }
}
