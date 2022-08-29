pub struct PublicKeyHash {
    hash: String,
}

impl PublicKeyHash {
    pub fn create(hash: String) -> Self {
        Self { hash }
    }

    pub fn get(&self) -> &String {
        &self.hash
    }
}

use std::fmt;

impl fmt::Display for PublicKeyHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[PublicKeyHash] {}", self.hash)
    }
}
