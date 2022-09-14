mod blockchain;
mod cashbook;
mod crypto;
mod transaction;

mod utils;

use std::env;

fn main() {
    // For debugging
    // env::set_var("RUST_BACKTRACE", "1");

    // Check the command line arguments to set option
    let args: Vec<String> = env::args().collect();

    let help_text = "Please run the program with an index to determine what should be done:\n\
        [1] Create a blockchain with one transaction\n\
        [2] Interactive view and creation of transactions in the blockchain\n\
        \n\
        Note that the second can only be done if the first one was already executed\n\
        as it depends on the saved blockchain.data file.";

    // Print help info if no argument is given
    if args.len() != 2 {
        println!("{}", help_text);
        return;
    }

    let option = &args[1];
    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: The argument should only be one number!\n");
            println!("{}", help_text);
            return;
        }
    };

    match option {
        1 => {
            let chain = utils::small_chain();
            chain.save("blockchain.data");
        }
        2 => utils::interface(),
        _ => {
            eprintln!("Error: Only 1 and 2 are valid choices!\n");
            println!("{}", help_text);
            return;
        }
    };
}
