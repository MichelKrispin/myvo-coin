mod blockchain;
mod crypto;
mod transaction;

mod dummy;

use blockchain::get_hash::GetHash;

const ALICE1_FILE: &str = "keys/alice1.keypair";
const BOB_FILE: &str = "keys/bob.keypair";
const ALICE2_FILE: &str = "keys/alice2.keypair";

const LEADING_ZEROS: usize = 1;

fn generate_keys() {
    // Generate
    let alice = crypto::keypair::Keypair::new();
    // Save
    alice.save(ALICE1_FILE.to_string());

    let bob = crypto::keypair::Keypair::new();
    bob.save(BOB_FILE.to_string());

    let alice = crypto::keypair::Keypair::new();
    alice.save(ALICE2_FILE.to_string());
}

fn small_chain() {
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
    //println!("\n{}\n", first_block);
}

fn main() {
    if false {
        generate_keys();
    }

    if true {
        small_chain();
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
