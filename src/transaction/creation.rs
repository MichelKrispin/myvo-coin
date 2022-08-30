use super::output;

const DEFAULT_CREATION_AMOUNT: u64 = 1;

pub struct Creation {
    pub output: output::Output,
    pub id_hash: String,
}

impl Creation {
    pub fn new(hash: String) -> Self {
        Self {
            output: output::Output::create(DEFAULT_CREATION_AMOUNT, hash),
            id_hash: String::from("the creation hash"),
        }
    }
}

use std::fmt;

impl fmt::Display for Creation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Creation]:{}\n  {}", self.id_hash, self.output,)
    }
}
