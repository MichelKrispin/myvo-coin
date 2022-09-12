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

use crate::blockchain;
use crate::blockchain::get_hash::GetHash;
use crate::crypto;
use crate::transaction;

pub fn generate_keys() {
    // Generate
    let alice = crypto::keypair::Keypair::new();
    // Save
    alice.save(alice.public_key().as_hex() + ".pk");

    let bob = crypto::keypair::Keypair::new();
    bob.save(bob.public_key().as_hex() + ".pk");

    let alice = crypto::keypair::Keypair::new();
    alice.save(alice.public_key().as_hex() + ".pk");
}

const LEADING_ZEROS: usize = 1;

const ALICE1_FILE: &str = "keys/hex_1.pk"; // Created in the generate_keys() function
const BOB_FILE: &str = "keys/hex_2.pk";
const ALICE2_FILE: &str = "keys/hex_3.pk";

pub fn small_chain() -> blockchain::BlockChain {
    // Load the keypair for Alice
    // let alice = crypto::keypair::Keypair::new();
    let alice = crypto::keypair::Keypair::load(ALICE1_FILE.to_string());
    let alice_public_hash = crypto::hash::Hash::create(alice.public_key().as_hex());
    println!("Alice pk:   {}", alice.public_key());
    println!("Alice hash: {}", alice_public_hash);

    // First block
    let first_block = {
        let cloned_hash = crypto::hash::Hash::clone(&alice_public_hash);
        blockchain::first_block::FirstBlock::new(cloned_hash)
    };

    // -----

    // Create the keypair for Bob
    // let bob = crypto::keypair::Keypair::new();
    let bob = crypto::keypair::Keypair::load(BOB_FILE.to_string());
    let bob_public_hash = crypto::hash::Hash::create(bob.public_key().as_hex());
    println!("Bob   pk:   {}", bob.public_key());
    println!("Bob   hash: {}", bob_public_hash);

    // Create a transaction from Alice to Bob
    let first_transaction = {
        // Connect the input to Alice creation coin
        let alice_input = {
            let validator = transaction::validator::Validator::create(alice, b"alice");
            let cloned_hash = crypto::hash::Hash::clone(&alice_public_hash);
            transaction::input::Input::create(cloned_hash, validator)
        };
        // That is the only input right now
        let inputs = vec![alice_input];
        // Then create the output for Bobs public hash
        let output = transaction::output::Output::create(1, bob_public_hash);

        // And create the new transaction with that
        transaction::Transaction::new(inputs, output, None)
    };

    // Then create a Creation for Alice, because she computes the block
    // For that alice gets a new keypair, because the old one was already used.
    // let alice = crypto::keypair::Keypair::new();
    let alice = crypto::keypair::Keypair::load(ALICE2_FILE.to_string());
    let alice_public_hash = crypto::hash::Hash::create(alice.public_key().as_hex());
    println!("Alice pk:   {}", alice.public_key());
    println!("Alice hash: {}", alice_public_hash);

    // Then create the block and insert all the created information
    let mut block = {
        let last_block_hash = first_block.hash();
        let transactions = vec![first_transaction];
        blockchain::block::Block::new(last_block_hash, alice_public_hash, transactions)
    };

    // And Compute the correct hash
    block.compute_hash(LEADING_ZEROS);

    println!("");
    let mut blockchain = blockchain::BlockChain::create(first_block);
    blockchain.add_block(block);
    println!("{}", blockchain);
    blockchain
}
