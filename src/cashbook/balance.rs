use super::receipt;

/// Contains a list of receipts and
/// the summed amount of them.
pub struct Balance {
    receipts: Vec<receipt::Receipt>,
    unused: Vec<String>,
    amount: u64,
}

impl Balance {
    pub fn empty() -> Self {
        Self {
            receipts: vec![],
            unused: vec![],
            amount: 0,
        }
    }

    pub fn create(receipts: Vec<receipt::Receipt>, unused: Vec<String>) -> Self {
        let mut amount = 0;
        for receipt in &receipts {
            amount += receipt.get_amount();
        }
        Self {
            receipts,
            unused,
            amount,
        }
    }

    pub fn add(&mut self, receipt: receipt::Receipt) {
        self.receipts.push(receipt)
    }

    pub fn get_amount(&self) -> &u64 {
        &self.amount
    }
}

use std::fmt;

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Balance]: {}\n", self.get_amount())?;
        for receipt in &self.receipts {
            write!(f, " -> {}", receipt)?;
        }
        if self.unused.len() > 0 {
            write!(f, "\n")?;
            for not_in_use in &self.unused {
                write!(f, " -> {} (unused public key)\n", not_in_use)?;
            }
        }
        write!(f, "\n")
    }
}
