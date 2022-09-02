use hex;
use sha3::{Digest, Sha3_512};

/// The length of a SHA3-512 hash is 64 bytes
pub const HASH_LENGTH: usize = 64;

/// A simple wrapper class around hashes.
/// Provides utility functions to generate hashes from values.
#[derive(Copy, Clone)]
pub struct Hash {
    /// The hash itself with the length corresponding to the used algorithm.
    hash: [u8; HASH_LENGTH],
}

impl Hash {
    /// Creates a new hash from the given string.
    ///
    /// # Arguments
    ///
    /// * `to_hash` - The string which will be hashed.
    pub fn create(to_hash: impl AsRef<[u8]>) -> Self {
        let mut hasher = Sha3_512::new();
        hasher.update(to_hash);
        Self {
            hash: hasher.finalize().into(),
        }
    }

    /// View the hash as a hex value.
    pub fn as_hex(&self) -> String {
        String::from(hex::encode(&self.hash))
    }

    // Get the bytes of this hash.
    pub fn as_bytes(&self) -> &[u8; HASH_LENGTH] {
        &self.hash
    }
}

use std::fmt;

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Hash] {}", self.as_hex())
    }
}
