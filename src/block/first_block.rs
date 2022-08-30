use super::transaction;

pub struct FirstBlock {
    creation: transaction::creation::Creation,
    id_hash: String,
}

impl FirstBlock {
    pub fn new(output_hash: String) -> Self {
        Self {
            creation: transaction::creation::Creation::new(output_hash),
            id_hash: String::from("the very first hash"),
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
