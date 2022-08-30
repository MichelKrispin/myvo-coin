use super::transaction;

use crate::crypto::hash;

pub struct FirstBlock {
    creation: transaction::creation::Creation,
    id_hash: hash::Hash,
}

impl FirstBlock {
    pub fn new(output_hash: hash::Hash) -> Self {
        Self {
            creation: transaction::creation::Creation::new(output_hash),
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
