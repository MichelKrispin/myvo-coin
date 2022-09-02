use super::output;
use crate::crypto::hash;

/// The default number of coins that will be created
/// when a new block is added to the chain.
const DEFAULT_CREATION_AMOUNT: u64 = 1;

/// A creation block that should be included
/// at the top of a block.
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
        Self {
            output: output::Output::create(DEFAULT_CREATION_AMOUNT, new_owner),
            id_hash: hash::Hash::create(String::from("the creation hash")),
        }
    }

    pub fn get_output(&self) -> &output::Output {
        &self.output
    }
}

use std::fmt;

impl fmt::Display for Creation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Creation]:{}\n  {}", self.id_hash, self.output,)
    }
}
