pub mod first_block;
pub mod get_hash;

use super::transaction;
use crate::crypto::hash;
use get_hash::GetHash;

/// A block in the blockchain that contains
/// multiple transactions, the hash of the last
/// block in the chain, one creation transaction
/// as the incentive for the block
/// and a new hash of this complete block.
pub struct Block {
    /// The hash of the last block.
    last_id_hash: hash::Hash,

    /// The new coin creation transaction.
    creation: transaction::creation::Creation,

    /// A list of transactions inside of this block.
    transactions: Vec<transaction::Transaction>,

    /// A nonce to change the hash
    nonce: u64,

    /// The hash of this block that has to follow a
    /// predefined set of rules (e.g. 3 leading zeros)
    id_hash: hash::Hash,
}

impl Block {
    /// Create a new block that refers to the
    /// block with the given hash.
    ///
    /// # Arguments
    /// * `last_id_hash` - The hash of the last block.
    /// * `creation_owner` - The owner that gets the newly created coins.
    pub fn new(
        last_id_hash: hash::Hash,
        creation_owner: hash::Hash,
        transactions: Vec<transaction::Transaction>,
    ) -> Self {
        // TODO: Actually hash that stuff and adjust the nonce
        Self {
            last_id_hash,
            creation: transaction::creation::Creation::new(creation_owner),
            transactions,
            nonce: 0,
            id_hash: hash::Hash::create(String::from("this blocks dummy hash")),
        }
    }

    /// Add a new transaction to the list of transactions
    ///
    /// # Arguments
    /// * `transaction` - A new transaction that is hopefully valid.
    pub fn add_transactions(&mut self, transaction: transaction::Transaction) {
        // TODO: Check if the transaction is valid
        self.transactions.push(transaction);
    }

    /// Compute the hash of this block.
    /// Return true, if the hash applies to the validation rules.
    pub fn compute_hash(&mut self) -> bool {
        // TODO: Actually... Compute the hash and adjust the nonce
        self.nonce += 1;
        self.id_hash = hash::Hash::create(String::from("a new hash"));
        false
    }
}

impl GetHash for Block {
    fn hash(&self) -> hash::Hash {
        self.id_hash
    }
}

use std::fmt;

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Block]:{}\n     -> {}\n",
            self.last_id_hash, self.id_hash
        )?;
        write!(f, "--------------------------\n")?;
        write!(f, "{}\n", self.creation)?;
        for transaction in &self.transactions {
            write!(f, "  ---\n{}\n", transaction)?;
        }
        write!(f, "--------------------------\n")
    }
}
