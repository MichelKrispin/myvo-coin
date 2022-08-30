pub mod first_block;

use super::transaction;

pub struct Block {
    last_id_hash: String,
    creation: transaction::creation::Creation,
    transactions: Vec<transaction::Transaction>,
    id_hash: String,
}

impl Block {
    pub fn new(last_id_hash: String) -> Self {
        Self {
            last_id_hash,
            creation: transaction::creation::Creation::new(String::from("new owner of creation")),
            transactions: vec![
                transaction::Transaction::empty(),
                transaction::Transaction::empty(),
            ],
            id_hash: String::from("this blocks hash"),
        }
    }
}

use std::fmt;

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Block]:{} -> {}\n", self.last_id_hash, self.id_hash)?;
        write!(f, "--------------------------\n")?;
        write!(f, "{}\n", self.creation)?;
        for transaction in &self.transactions {
            write!(f, "  ---\n{}\n", transaction)?;
        }
        write!(f, "--------------------------\n")
    }
}
