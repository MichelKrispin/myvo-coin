use super::public_key_hash;

pub struct Output {
    amount: u64,
    owner: public_key_hash::PublicKeyHash,
}

impl Output {
    pub fn create(amount: u64, hash: String) -> Self {
        Self {
            amount,
            owner: public_key_hash::PublicKeyHash::create(hash),
        }
    }
}

use std::fmt;

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Output] {} of {}", self.amount, self.owner.get())
    }
}
