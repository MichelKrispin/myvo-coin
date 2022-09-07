pub mod block;
pub mod first_block;
pub mod get_hash;

use super::transaction;
use crate::{crypto::hash, transaction::output};

const LEADING_ZEROS: usize = 1;

/// A blockchain containing a list of blocks.
pub struct BlockChain {
    /// The very first block
    first_block: first_block::FirstBlock,

    /// A list of transactions inside of this block.
    blocks: Vec<block::Block>,
}

impl BlockChain {
    /// Create a new block that refers to the
    /// block with the given hash.
    ///
    /// # Arguments
    /// * `first_block` - The very first block.
    pub fn create(first_block: first_block::FirstBlock) -> Self {
        Self {
            first_block,
            blocks: vec![],
        }
    }

    /// This adds the block to the blockhain.
    /// Includes all the error checking and validation code.
    /// Checks for input validator correctness
    /// and correct hash values.
    pub fn add_block(&mut self, block: block::Block) -> bool {
        // This is the place for all error checking stuff!

        // Check each transaction inside this block
        for transaction in block.get_transactions() {
            let mut input_amount: u64 = 0;
            let transaction_amount = transaction.get_output().get_amount();

            // Check each input for correctness
            for input in transaction.get_inputs() {
                // Check if the output that this input refers to actually exist and wasn't spended
                let output_reference_hash = input.get_output_reference();
                let output = self
                    .get_valid_output(*output_reference_hash)
                    .expect("Output referred to by an input has already been used!");
                // Sum the amount of the inputs together
                input_amount += output.get_amount();

                // Check if the validator is actually correct (proof ownership)
                // For this, the validators public key has to be hashed and
                // then has to equal the outputs hash
                let validator = input.get_validator();
                let validator_public_key = validator.get_public_key();
                let validator_public_key_hash = hash::Hash::create(validator_public_key.as_hex());
                if validator_public_key_hash.as_hex() != output.get_owner_hash().as_hex() {
                    panic!("The output is not owned by this validator (hashed keys don't equal)");
                }

                // Also, the signature had to be made with the corresponding private key
                let message = b"alice"; // TODO: Change this!!!
                if !validator_public_key.check(message, validator.get_signature()) {
                    panic!("The output is not owned by this validator (the signature isn't valid)");
                }
            }

            // Check if the summed amount is correct
            if input_amount != transaction_amount {
                panic!("Inputs amount doesn't equal the transaction amount");
            }
        }

        // And finally check whether the hash is valid\
        let block_hash = block.get_hash();
        let hash_bytes = block_hash.as_bytes();
        for i in 0..LEADING_ZEROS {
            if hash_bytes[i] != 0 {
                panic!(
                    "The blocks hash doesn't start with {} zeros:\n[{}, {}, ...\n",
                    LEADING_ZEROS, hash_bytes[0], hash_bytes[1]
                );
            }
        }

        // Some information printing on the beginning of the hash.
        println!(
            "The block starts with [{}, {}, {}, {}, ...\n",
            hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3]
        );

        // And then do the hash and compare that to the one that was created
        if block_hash.as_hex() != block.get_hash().as_hex() {
            panic!("The blocks hash was manipulated!");
        }

        self.blocks.push(block);
        true
    }

    /// Search through all blocks in the output and return the
    /// output if it wasn't already used.
    /// Panics, if the output couldn't been found.
    pub fn get_valid_output(&self, output_hash: hash::Hash) -> Option<&output::Output> {
        // First try to get it
        let output = self.get_output(output_hash);

        // Then search through all blocks and see if it has already been used
        for block in &self.blocks {
            for transaction in block.get_transactions() {
                for input in transaction.get_inputs() {
                    let reference_hash = input.get_output_reference();
                    if reference_hash.as_hex() == output_hash.as_hex() {
                        return None;
                    }
                }
            }
        }

        return Some(output);
    }

    /// Searches through all outputs in the blockchain
    /// and return the output if it exists.
    /// Otherwise it panics!
    fn get_output(&self, output_hash: hash::Hash) -> &output::Output {
        // First check the first blocks output
        let transaction_output = self.first_block.get_output();
        let owner_hash = transaction_output.get_owner_hash();
        if owner_hash.as_hex() == output_hash.as_hex() {
            return transaction_output;
        }

        // Then check all the other blocks
        for block in &self.blocks {
            // Check the creation transactions
            {
                let creation = block.get_creation();
                let creation_output = creation.get_output();
                let creation_hash = creation_output.get_owner_hash();
                if creation_hash.as_hex() == output_hash.as_hex() {
                    return transaction_output;
                }
            }

            // And then all other transactions
            for transaction in block.get_transactions() {
                let transaction_output = transaction.get_output();
                let owner_hash = transaction_output.get_owner_hash();
                if owner_hash.as_hex() == output_hash.as_hex() {
                    return transaction_output;
                }
            }
        }
        panic!("Could not find the output in the blockchain");
    }
}

use std::fmt;

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " --- [BlockChain] ---\n\n{}\n", &self.first_block)?;
        for block in &self.blocks {
            write!(f, "_______\n{}\n", block)?;
        }
        Ok(())
    }
}
