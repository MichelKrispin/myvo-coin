use super::public_key_hash;

pub struct Output {
    pub amount: u64,
    pub owner: public_key_hash::PublicKeyHash,
}

// impl Output {}

use std::fmt;

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Output] {} of {}", self.amount, self.owner.get())
    }
}
