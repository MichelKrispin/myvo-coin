use super::get_hash;
use super::transaction;

use serde;

use crate::crypto::hash;
use crate::transaction::output;

/// The very first block in the chain.
/// As it has no predecessor, it looks a little bit different.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FirstBlock {
    /// The first creation to get some coins.
    creation: transaction::creation::Creation,

    /// The hash of this block to have an address.
    id_hash: hash::Hash,
}

impl FirstBlock {
    /// Create the very first block.
    ///
    /// # Arguments
    /// * `first_owner_hash` - The hash of the owner of the very first coins.
    pub fn new(first_owner_hash: hash::Hash) -> Self {
        Self {
            creation: transaction::creation::Creation::new(first_owner_hash),
            id_hash: hash::Hash::create(String::from("the very first hash")),
        }
    }

    pub fn get_output(&self) -> &output::Output {
        &self.creation.get_output()
    }
}

impl get_hash::GetHash for FirstBlock {
    fn hash(&self) -> hash::Hash {
        hash::Hash::clone(&self.id_hash)
    }
}

use std::fmt;

impl fmt::Display for FirstBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[First Block]:{}\n", self.id_hash)?;
        write!(f, "--------------------------\n")?;
        write!(f, "{}\n", self.creation)?;
        write!(f, "--------------------------\n")
    }
}
