//
// Tests and references on how the encryption process works.
//

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

//
// Utilities to create a first block and a simple blockchain.
//

use crate::blockchain;
use crate::blockchain::get_hash::GetHash;
use crate::crypto;
use crate::transaction;

/// Generate 3 new keys and save them in the keys folder
/// and return them for later use.
fn generate_keys() -> (
    crypto::keypair::Keypair,
    crypto::keypair::Keypair,
    crypto::keypair::Keypair,
) {
    // Generate and save
    let key = crypto::keypair::Keypair::new();
    let filename = format!("keys/{}.pk", key.public_key().as_hex());
    key.save(filename);
    let key_1 = key;

    let key = crypto::keypair::Keypair::new();
    let filename = format!("keys/{}.pk", key.public_key().as_hex());
    key.save(filename);
    let key_2 = key;

    let key = crypto::keypair::Keypair::new();
    let filename = format!("keys/{}.pk", key.public_key().as_hex());
    key.save(filename);
    let key_3 = key;

    (key_1, key_2, key_3)
}

const LEADING_ZEROS: usize = 1;

/// Create a simple blockchain with one transaction.
pub fn small_chain() -> blockchain::BlockChain {
    let (key_1, key_2, key_3) = generate_keys();

    // Load the keypair for Alice
    let alice = key_1;
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
    let bob = key_2;
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
    let alice = key_3;
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

//
// Utitilities to view and create transactions.
//

use crate::cashbook;

use std::io;

/// Helper function to ask for input until a 1 or 2 is returned.
fn get_1_2_as_input() -> u32 {
    loop {
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) if num == 1 || num == 2 => num,
            Ok(_) => {
                println!("Please only type in 1 or 2.\n");
                continue;
            }
            Err(_) => {
                println!("Please only type numbers.\n");
                continue;
            }
        };
        break option;
    }
}

/// A simple interface that makes it possible to read the current
/// amount of owned coins and to make transaction to newly created keypairs.
/// Depends on already existings keys in a directory called 'keys'
/// and an existent blockchain called 'blockchain.data'.
pub fn interface() {
    // Open up a cash book with keys and the saved blockchain
    let mut cash_book = {
        // Load up the blockchain data
        let blockchain = blockchain::BlockChain::load("blockchain.data");
        cashbook::CashBook::open(String::from("keys"), blockchain)
    };

    loop {
        println!(
            "\n\n\x1b[1m      --- [ myvo-coin ] --- \n\
            .....What do you want to do?.....\x1b[0m\n\n\
            [1] Print Cash Book information\n\
            [2] Create a new keypair and print the key hash\n\
            [3] Create a new transaction\n\
            [4] Quit\n\
            "
        );

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please only type numbers.\n");
                continue;
            }
        };

        match option {
            // Print Cash Book information
            1 => println!("{}", cash_book),

            // Create a new keypair and print the key hash
            2 => {
                let public_key_as_hex = cash_book.create_keypair();
                println!(
                    "Created a new keypair with this public key:\n{}",
                    public_key_as_hex
                );
            }

            // Create a new transaction
            3 => {
                let (inputs, receiver_public_key_hash, amount, creation_public_key_hash) = loop {
                    // First get the inputs to use for the transaction
                    let inputs = {
                        let mut inputs: Vec<transaction::input::Input> = vec![];

                        loop {
                            let add_another = loop {
                                // Ask for another input
                                println!(
                                    "-> Do you want to add an input?\n\
                                    [1] Yes\n\
                                    [2] No"
                                );
                                let option = get_1_2_as_input();
                                match &option {
                                    1 => break true,
                                    _ => break false,
                                }
                            };
                            // Break if that was false
                            if !add_another {
                                break;
                            }

                            // Load the corresponding input keypair file
                            let keypair = loop {
                                println!("-> Please input the name of the input keypair file.");
                                let mut filename = String::new();
                                io::stdin()
                                    .read_line(&mut filename)
                                    .expect("Failed to read line");
                                let filename = filename.trim().to_owned();
                                match crypto::keypair::Keypair::load(filename) {
                                    Ok(keypair) => break keypair,
                                    Err(error) => {
                                        println!("Error with the input file: {}", error);
                                        continue;
                                    }
                                }
                            };
                            // Create a validator from it to make an input
                            let validator =
                                transaction::validator::Validator::create(keypair, b"alice");
                            let public_key_hash = {
                                let public_key = validator.get_public_key();
                                crypto::hash::Hash::create(public_key.as_hex())
                            };
                            // Then create a new input and add it to the list
                            let input =
                                transaction::input::Input::create(public_key_hash, validator);
                            inputs.push(input);
                        }

                        inputs
                    };

                    // Then get the receiver
                    println!("-> Please input the public key as hex of the recipient.");
                    let mut option = String::new();
                    io::stdin()
                        .read_line(&mut option)
                        .expect("Failed to read line");
                    let receiver_public_key_as_hex = option;

                    // Then the amount
                    let amount = loop {
                        println!("-> Please input the amount to be sent.");
                        let mut option = String::new();
                        io::stdin()
                            .read_line(&mut option)
                            .expect("Failed to read line");

                        let amount: u32 = match option.trim().parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Please only type numbers.\n");
                                continue;
                            }
                        };
                        break amount;
                    };

                    // Then the receiver of the creation
                    println!("-> Please input the public key as hex of the creation.");
                    let mut option = String::new();
                    io::stdin()
                        .read_line(&mut option)
                        .expect("Failed to read line");
                    let creation_public_key_as_hex = option;

                    let receiver_public_key_hash =
                        crypto::hash::Hash::create(receiver_public_key_as_hex.trim());
                    let creation_public_key_hash =
                        crypto::hash::Hash::create(creation_public_key_as_hex.trim());
                    // Then ask for correctness
                    println!(
                        "\n-> Is this correct?\n\
                    > Receiver public key as hex: {}\n\
                    > Amount:                     {}\n\
                    > Creation public key as hex: {}\n\n\
                    [1] Correct\n\
                    [2] Change
                    ",
                        receiver_public_key_as_hex.trim(),
                        amount,
                        creation_public_key_as_hex.trim(),
                    );

                    // Loop until a 1 or 2 was typed
                    let option = get_1_2_as_input();
                    // Then return the values if its a 1 or ask again if it was a 2
                    match &option {
                        1 => {
                            break (
                                inputs,
                                receiver_public_key_hash,
                                u64::from(amount),
                                creation_public_key_hash,
                            )
                        }
                        _ => continue,
                    };
                };

                // Then build the transaction into a block and add it to the blockchain
                let mut blockchain = blockchain::BlockChain::load("blockchain.data");
                let mut block = {
                    // Create the output from the given informaiton
                    let output =
                        transaction::output::Output::create(amount, receiver_public_key_hash);

                    // Then create a new transaction
                    let transactions: Vec<transaction::Transaction> =
                        vec![transaction::Transaction::new(inputs, output, None)];

                    // Get the last hash from the blockchain
                    let last_id_hash = blockchain.get_last_block_hash();
                    let last_id_hash = crypto::hash::Hash::clone(last_id_hash);

                    // And create the block
                    blockchain::block::Block::new(
                        last_id_hash,
                        creation_public_key_hash,
                        transactions,
                    )
                };
                // Comput the correct hash
                block.compute_hash(LEADING_ZEROS);

                // Then add it to the blockchain
                blockchain.add_block(block);
                blockchain.save("blockchain.data");

                // And update the cash book
                cash_book.update_blockchain(blockchain);
            }

            // Quit
            4 => {
                println!("Quitting...");
                break;
            }
            _ => println!("This number is not a valid option."),
        }

        println!("\n");
    }
}
