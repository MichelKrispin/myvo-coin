use crate::crypto::keypair;

pub struct Receipt {
    amount: u64,
    keypair: keypair::Keypair,
}

impl Receipt {
    pub fn create(amount: u64, keypair: keypair::Keypair) -> Self {
        Self { amount, keypair }
    }

    pub fn get_amount(&self) -> &u64 {
        &self.amount
    }
}

use std::fmt;

impl fmt::Display for Receipt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Receipt]: {} ({})\n", self.amount, self.keypair)
    }
}
