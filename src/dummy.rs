use ed25519_dalek::{Keypair, Signature, Signer};
use hex;
use rand::rngs::OsRng;

pub fn signature_test() {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public.as_bytes();
    println!("{}", hex::encode(public_key));
    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signature: Signature = keypair.sign(message);
    println!("{}", signature.to_string());
    println!("{}", keypair.verify(message, &signature).is_ok());
}

use sha3::{Digest, Sha3_512};

pub fn hash_test() {
    let mut hasher = Sha3_512::new();
    hasher.update(b"abc");
    let result = hasher.finalize();
    println!("{}", hex::encode(result));
}

pub fn hash_public_key() {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public.as_bytes();

    let mut hasher = Sha3_512::new();
    hasher.update(public_key);
    let result: [u8; 64] = hasher.finalize().into();
    println!("{}", hex::encode(result));
}
