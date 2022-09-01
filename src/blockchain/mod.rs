pub mod block;
pub mod first_block;
pub mod get_hash;

use super::transaction;
use crate::crypto::hash;
// use get_hash::GetHash;

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

        // TODO: First check if all inputs belong to existing outputs
        //       and that the summed amount is correct
        //       and that the validator is correct

        self.blocks.push(block);
        true
    }

    /// Searches through all outputs in the blockchain and
    /// returns true if found, else false.
    fn get_output(&self, output_hash: hash::Hash) -> bool {
        for block in &self.blocks {
            for transaction in block.get_transactions() {
                let owner_hash = transaction.get_output().get_owner_hash();
                if owner_hash.as_hex() == output_hash.as_hex() {
                    return true;
                }
            }
        }
        false
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
