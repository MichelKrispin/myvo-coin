mod blockchain;
mod cashbook;
mod crypto;
mod transaction;

mod dummy;

use blockchain::get_hash::GetHash;

const ALICE1_FILE: &str =
    "keys/028f9082911a0e1b7d3c3dc86ba46c421004474c68f0b11b462ae361be82627b.pk";
const BOB_FILE: &str = "keys/211c92696588cb84e605be62630fb43ea6b412aaa202e1fb0a89b027a9f8a3eb.pk";
const ALICE2_FILE: &str =
    "keys/c5c947470194421f99e7f3ed5d99b11cbec158a908526236dd07165532544b63.pk";

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
    let blockchain = small_chain();
    let cash_book = cashbook::CashBook::open(String::from("keys"), blockchain);
    println!("{}", cash_book);
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
