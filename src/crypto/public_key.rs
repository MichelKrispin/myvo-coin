use super::signature;

use ed25519_dalek;
use ed25519_dalek::Verifier;
use hex;

/// A wrapper for a public key implementation.
#[derive(Copy, Clone)]
pub struct PublicKey {
    key: ed25519_dalek::PublicKey,
}

impl PublicKey {
    /// Create a public key from a pub
    pub fn create_from(key: ed25519_dalek::PublicKey) -> Self {
        Self { key }
    }

    /// Returns the bytes of this public key.
    pub fn as_bytes(&self) -> &[u8; ed25519_dalek::PUBLIC_KEY_LENGTH] {
        self.key.as_bytes()
    }

    /// Returns the hex representation of this public key.
    pub fn as_hex(&self) -> String {
        hex::encode(self.key.as_bytes())
    }

    /// Check that the signature is actually the message signed
    /// with the correct private key.
    pub fn check(&self, message: &[u8], signature: &signature::Signature) -> bool {
        let signed_message = signature.signature();
        self.key.verify(&message, &signed_message).is_ok()
    }
}

use std::fmt;

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Public Key] {}", self.as_hex())
    }
}
