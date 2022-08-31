mod block;
mod crypto;
mod transaction;

fn block_test() {
    // Transaction
    let transaction = transaction::Transaction::empty();
    println!("{}", transaction);
    println!("=====\n");

    // First block
    let first_block = block::first_block::FirstBlock::new(crypto::hash::Hash::create(
        String::from("output hash of first block"),
    ));
    println!("{}", first_block);
    println!("=====\n");

    // Block
    let mut block = block::Block::new(
        crypto::hash::Hash::create(String::from("the very first hash")),
        crypto::hash::Hash::create(String::from("new owner of creation")),
    );
    block.add_transactions(transaction::Transaction::empty());
    block.compute_hash();
    println!("{}", block);
}

use ed25519_dalek::{Keypair, Signature, Signer};
use hex;
use rand::rngs::OsRng;

fn signature_test() {
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

fn hash_test() {
    let mut hasher = Sha3_512::new();
    hasher.update(b"abc");
    let result = hasher.finalize();
    println!("{}", hex::encode(result));
}

fn hash_public_key() {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public.as_bytes();

    let mut hasher = Sha3_512::new();
    hasher.update(public_key);
    let result: [u8; 64] = hasher.finalize().into();
    println!("{}", hex::encode(result));
}

fn main() {
    if true {
        block_test();
    }

    if false {
        signature_test();
    }

    if false {
        hash_test();
    }

    if false {
        hash_public_key();
    }
}
