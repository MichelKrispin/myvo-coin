use super::get_hash;
use super::transaction;
use crate::crypto::hash;

use std::time;

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
        Self {
            last_id_hash,
            creation: transaction::creation::Creation::new(creation_owner),
            transactions,
            nonce: 0,
            id_hash: hash::Hash::create(String::from("this blocks dummy hash")),
        }
    }

    /// Add a new transaction to the list of transactions.
    /// Doesn't check whether the transactions is valid!
    ///
    /// # Arguments
    /// * `transaction` - A new transaction that is hopefully valid.
    pub fn add_transaction(&mut self, transaction: transaction::Transaction) {
        self.transactions.push(transaction);
    }

    /// Get the transaction.
    pub fn get_transactions(&self) -> &Vec<transaction::Transaction> {
        &self.transactions
    }

    pub fn as_bytes(&self, with_hash: bool) -> Vec<u8> {
        // Flatten the block into an array such that it can be hashed
        let mut block: Vec<u8> = Vec::new();

        // First the last hash
        let bytes = self.last_id_hash.as_bytes();
        block.extend(bytes);

        // Then the creation
        let bytes = self.creation.as_bytes();
        block.extend(bytes);

        // Then the transactions
        for transaction in &self.transactions {
            let bytes = transaction.as_bytes();
            block.extend(bytes);
        }

        // Then the nonce
        let bytes = self.nonce.to_be_bytes();
        block.extend(bytes);

        if with_hash {
            block.extend(self.id_hash.as_bytes());
        }

        block
    }

    /// Compute the hash of this block.
    /// Return true, if the hash has `leading_zeros` zeros in front.
    pub fn compute_hash(&mut self, leading_zeros: usize) {
        if leading_zeros > hash::HASH_LENGTH {
            panic!("The leading zeros cannot be greater than the hash length!");
        }

        // Compute the time
        let begin = time::Instant::now();

        // Then loop until a correct hash is found.
        let mut loop_counter = 0;
        loop {
            print!("{}\r", loop_counter);

            // Check if the leading zeros match.
            let mut leading_zeros_match = true;
            let hash_bytes = self.id_hash.as_bytes();
            for i in 0..leading_zeros {
                if 0 != match hash_bytes.get(i) {
                    Some(byte) => *byte,
                    None => 0,
                } {
                    leading_zeros_match = false;
                    break;
                }
            }

            // If the hash starts with the right amount of zeros, then break.
            if leading_zeros_match {
                break;
            }

            // Otherwise adjust the nonce and recompute the hash.
            self.nonce += 1;
            // let bytes = self.nonce.to_be_bytes();
            let bytes = self.as_bytes(false);
            self.id_hash = hash::Hash::create(bytes);

            loop_counter += 1;
        }

        // Print the elapsed time.
        let elapsed_time = begin.elapsed();
        println!("{} tries in {}ms", loop_counter, elapsed_time.as_millis());
    }

    pub fn get_hash(&self) -> &hash::Hash {
        &self.id_hash
    }
}

impl get_hash::GetHash for Block {
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
