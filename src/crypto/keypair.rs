use super::public_key;
use super::signature;

use serde;

use ed25519_dalek;
use ed25519_dalek::Signer;
use rand::rngs;

use std::fs;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Keypair {
    keypair: ed25519_dalek::Keypair,
    public_key: public_key::PublicKey,
}

impl Keypair {
    /// Generate a new random keypair.
    pub fn new() -> Self {
        let mut csprng = rngs::OsRng {};
        let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let public_key = public_key::PublicKey::create_from(keypair.public);
        Self {
            keypair,
            public_key,
        }
    }

    /// Load a keypair from the given file.
    /// Should have been stored previously with
    /// the `save` function.
    pub fn load(filename: String) -> Result<Self, String> {
        let data = match fs::read(filename) {
            Ok(data) => data,
            Err(_) => return Err(String::from("Could not open keypair file")),
        };
        Ok(Keypair::from_bytes(&data))
    }

    /// Saves this keypair to a file as a raw byte string.
    pub fn save(&self, filename: String) {
        let data = self.keypair.to_bytes();
        fs::write(filename, data).expect("Unable to write keypair file");
    }

    /// Wrapper for creating a keypair from the given bytes.
    fn from_bytes(bytes: &Vec<u8>) -> Self {
        let keypair =
            ed25519_dalek::Keypair::from_bytes(bytes).expect("Cannot create keypair from bytes");
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
