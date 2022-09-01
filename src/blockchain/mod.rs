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
        // TODO: This is the place for all error checking stuff!
        self.blocks.push(block);
        true
    }

    /// Searches through all outputs in the blockchain and
    /// returns it if found.
    /// Otherwise returns None.
    pub fn get_output(&self, output_hash: hash::Hash) -> Option<transaction::output::Output> {
        // TODO: Search all blocks and all transaction for the output
        None
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
