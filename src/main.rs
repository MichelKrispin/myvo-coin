mod block;
mod transaction;

// https://docs.rs/ed25519/latest/ed25519/

// use ed25519::signature::{Signer, Verifier};
use ed25519_dalek::{Keypair, Signature};
use rand::rngs::OsRng;

fn main() {
    // First some tests here
    let transaction = transaction::Transaction::empty();
    println!("{}", transaction);
    println!("=====\n");
    let first_block =
        block::first_block::FirstBlock::new(String::from("output hash of first block"));
    println!("{}", first_block);
    println!("=====\n");
    let block = block::Block::new(String::from("the very first hash"));
    println!("{}", block);

    // Then some crypto tests here
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
}
