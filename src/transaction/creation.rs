use super::output;
use crate::crypto::hash;

const DEFAULT_CREATION_AMOUNT: u64 = 1;

pub struct Creation {
    pub output: output::Output,
    pub id_hash: hash::Hash,
}

impl Creation {
    pub fn new(hash: hash::Hash) -> Self {
        Self {
            output: output::Output::create(DEFAULT_CREATION_AMOUNT, hash),
            id_hash: hash::Hash::create(String::from("the creation hash")),
        }
    }
}

use std::fmt;

impl fmt::Display for Creation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Creation]:{}\n  {}", self.id_hash, self.output,)
    }
}
