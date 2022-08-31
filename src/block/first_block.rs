use super::transaction;

use crate::crypto::hash;

/// The very first block in the chain.
/// As it has no predecessor, it looks a little bit different.
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
    /// first_owner_hash: The hash of the owner of the very first coins.
    pub fn new(first_owner_hash: hash::Hash) -> Self {
        Self {
            creation: transaction::creation::Creation::new(first_owner_hash),
            id_hash: hash::Hash::create(String::from("the very first hash")),
        }
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
