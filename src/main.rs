mod blockchain;
mod cashbook;
mod crypto;
mod transaction;

mod dummy;

use blockchain::get_hash::GetHash;

const ALICE1_FILE: &str =
    "keys/634a05f117da18c0b0803290f0c5980814bd3886969a4a561d60e4d35749b863.pk";
const BOB_FILE: &str = "keys/d33d200d86250f3108dd36e60919203214431f378a6d3eb9c1106dc27f98e8cc.pk";
const ALICE2_FILE: &str =
    "keys/98b89be19e0b64aac6ce046c659303afc19e5dc79a58339115d8c782c2b5a946.pk";

const LEADING_ZEROS: usize = 1;

fn generate_keys() {
    // Generate
    let alice = crypto::keypair::Keypair::new();
    // Save
    alice.save(alice.public_key().as_hex() + ".pk");

    let bob = crypto::keypair::Keypair::new();
    bob.save(bob.public_key().as_hex() + ".pk");

    let alice = crypto::keypair::Keypair::new();
    alice.save(alice.public_key().as_hex() + ".pk");
}

fn small_chain() -> blockchain::BlockChain {
    // Load the keypair for Alice
    // let alice = crypto::keypair::Keypair::new();
    let alice = crypto::keypair::Keypair::load(ALICE1_FILE.to_string());
    let alice_public_hash = crypto::hash::Hash::create(alice.public_key().as_hex());
    println!("Alice pk:   {}", alice.public_key());
    println!("Alice hash: {}", alice_public_hash);

    // First block
    let first_block = blockchain::first_block::FirstBlock::new(alice_public_hash);

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
            transaction::input::Input::create(alice_public_hash, validator)
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

fn cash_book() {
    // Create a small chain
    let blockchain = small_chain();

    // Open up a cash book with keys and the created chain
    let cash_book = cashbook::CashBook::open(String::from("keys"), blockchain);
    println!("\n\n{}", cash_book);
}

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    if false {
        generate_keys();
    }

    if false {
        let _ = small_chain();
    }

    if true {
        cash_book();
    }

    if false {
        dummy::signature_test();
    }

    if false {
        dummy::hash_test();
    }

    if false {
        dummy::hash_public_key();
    }
}
