use super::public_key;
use super::signature;

use ed25519_dalek;
use ed25519_dalek::Signer;
use rand::rngs;

pub struct Keypair {
    keypair: ed25519_dalek::Keypair,
    public_key: public_key::PublicKey,
}

impl Keypair {
    pub fn new() -> Self {
        let mut csprng = rngs::OsRng {};
        let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let public_key = public_key::PublicKey::create_from(keypair.public);
        Self {
            keypair,
            public_key,
        }
    }

    pub fn sign(&self, message: &[u8]) -> signature::Signature {
        signature::Signature::create(self.keypair.sign(message))
    }

    pub fn public_key(&self) -> public_key::PublicKey {
        self.public_key
    }
}

use std::fmt;

impl fmt::Display for Keypair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Keypair] {}", self.public_key)
    }
}
