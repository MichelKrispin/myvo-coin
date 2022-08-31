use ed25519_dalek;

pub struct Signature {
    signature: ed25519_dalek::Signature,
}

impl Signature {
    pub fn create(signature: ed25519_dalek::Signature) -> Self {
        Self { signature }
    }
    pub fn signature(&self) -> ed25519_dalek::Signature {
        self.signature
    }
}

use std::fmt;

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Signature] {}", self.signature.to_string())
    }
}
