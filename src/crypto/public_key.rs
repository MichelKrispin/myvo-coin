use super::hash;

use ed25519_dalek::PublicKey as PublicKey_ed25519;
use hex;

pub struct PublicKey {
    key: PublicKey_ed25519,
}

impl PublicKey {
    /// Hashes this key and returns the hash.
    fn hash(&self) -> hash::Hash {
        hash::Hash::create(self.key.as_bytes())
    }

    /// Returns the hex representation of this public key.
    fn as_hex(&self) -> String {
        hex::encode(self.key.as_bytes())
    }
}

use std::fmt;

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Public Key]: {}", self.as_hex())
    }
}
