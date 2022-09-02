use ed25519_dalek;

pub const SIGNATURE_LENGTH: usize = ed25519_dalek::SIGNATURE_LENGTH;

/// A very small signature wrapper to have a uniform desing.
pub struct Signature {
    /// The actual signature struct that this struct is wrapping up.
    signature: ed25519_dalek::Signature,
}

impl Signature {
    /// Create a new signature.
    pub fn create(signature: ed25519_dalek::Signature) -> Self {
        Self { signature }
    }

    /// Return the signature that is inside.
    pub fn signature(&self) -> ed25519_dalek::Signature {
        self.signature
    }

    /// Get the flat version of this signature.
    pub fn as_bytes(&self) -> [u8; SIGNATURE_LENGTH] {
        self.signature.to_bytes()
    }
}

use std::fmt;

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Signature] {}", self.signature.to_string())
    }
}
