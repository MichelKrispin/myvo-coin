mod blockchain;
mod cashbook;
mod crypto;
mod transaction;

mod dummy;

use std::io;

fn cash_book() {
    // Create a small chain
    let blockchain = blockchain::BlockChain::load("blockchain.data");

    // Open up a cash book with keys and the created chain
    let cash_book = cashbook::CashBook::open(String::from("keys"), blockchain);
    println!("\n\n{}", cash_book);
}

fn interface() {
    // Load up the blockchain data
    let blockchain = blockchain::BlockChain::load("blockchain.data");

    // Open up a cash book with keys and the created chain
    let mut cash_book = cashbook::CashBook::open(String::from("keys"), blockchain);

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
                let public_key_hash = cash_book.create_keypair();
                println!(
                    "Created a new keypair with this public key hash:\n{}",
                    public_key_hash
                );
            }

            // Create a new transaction
            3 => {
                let (receiver_public_key_hash, amount, creation_public_key_hashloop) = loop {
                    // First get the receiver
                    println!("Please input the public key hash of the recipient.\n");
                    let mut option = String::new();
                    io::stdin()
                        .read_line(&mut option)
                        .expect("Failed to read line");
                    let receiver_public_key_hash = option;

                    // Then the amount
                    let amount = loop {
                        println!("Please input the amount to be sent.\n");
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
                    println!("Please input the public key hash of the creation recipient.\n");
                    let mut option = String::new();
                    io::stdin()
                        .read_line(&mut option)
                        .expect("Failed to read line");
                    let creation_public_key_hash = option;

                    // Then ask for correctness
                    println!("Is this correct?\n");

                    // Loop until a 1 or 2 was typed
                    let option = loop {
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
                    };

                    // Then return the values if its a 1 or ask again if it was a 2
                    match option {
                        1 => break (receiver_public_key_hash, amount, creation_public_key_hash),
                        _ => continue,
                    }
                };

                // Then create a new transaction
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

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    if true {
        interface();
    }

    if false {
        cash_book();
    }

    if false {
        dummy::generate_keys();
    }

    if false {
        let chain = dummy::small_chain();
        chain.save("blockchain.data");
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
