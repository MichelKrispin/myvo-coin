use crate::crypto::{keypair, public_key, signature};

/// The validator to proof ownership of a given output.
pub struct Validator {
    /// The signature that was made with the private key that
    /// corresponds to the public key.
    signature: signature::Signature,

    // The corresponding public key to validate the signature.
    public_key: public_key::PublicKey,
}

impl Validator {
    pub fn create(keypair: keypair::Keypair, message: &[u8]) -> Self {
        Self {
            signature: keypair.sign(message),
            public_key: keypair.public_key(),
        }
    }
}

use std::fmt;

impl fmt::Display for Validator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Validator] {} : {}", self.signature, self.public_key,)
    }
}
