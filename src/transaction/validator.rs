use crate::crypto::{keypair, public_key, signature};

use std::convert::TryInto;

/// The validator length in bytes.
pub const VALIDATOR_LENGTH: usize = signature::SIGNATURE_LENGTH + public_key::PUBLIC_KEY_LENGTH;

/// The validator to proof ownership of a given output.
pub struct Validator {
    /// The signature that was made with the private key that
    /// corresponds to the public key.
    signature: signature::Signature,

    // The corresponding public key to validate the signature.
    public_key: public_key::PublicKey,
}

impl Validator {
    /// Create a validator to proof ownership over a given output
    /// by passing on the keypair that was used to create the
    /// output public key hash.
    pub fn create(keypair: keypair::Keypair, message: &[u8]) -> Self {
        Self {
            signature: keypair.sign(message),
            public_key: keypair.public_key(),
        }
    }

    /// Get the signature of this validator.
    pub fn get_signature(&self) -> &signature::Signature {
        &self.signature
    }

    /// Get the public key of this validator.
    pub fn get_public_key(&self) -> &public_key::PublicKey {
        &self.public_key
    }

    /// Return a flat version of bytes of everything that this validator contains.
    pub fn as_bytes(&self) -> [u8; VALIDATOR_LENGTH] {
        let v1 = self.public_key.as_bytes();
        let v2 = &self.signature.as_bytes();
        let whole: Vec<u8> = v1.iter().chain(v2.iter()).map(|v| *v).collect();
        let whole: [u8; VALIDATOR_LENGTH] = whole.try_into().unwrap();
        whole
    }
}

use std::fmt;

impl fmt::Display for Validator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Validator]\n   {}\n   {}",
            self.signature, self.public_key,
        )
    }
}
