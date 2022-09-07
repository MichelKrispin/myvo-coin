pub mod creation;
pub mod input;
pub mod output;
pub mod validator;

use crate::crypto::hash;

use serde;

/// A transaction that maps one or more inputs to one output
/// and one optional change output.
/// It is uniquely defined by the hash of all of its input values
/// serialized into a long list of bytes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    /// A list of inputs that refer from previous transactions output.
    pub inputs: Vec<input::Input>,

    /// One output to the new owner of the coins defined in this output.
    pub output: output::Output,

    /// One optional change output, if there is any.
    pub change: Option<output::Output>,

    /// A hash value of all previous data that uniquely identifies this transactions.
    pub id_hash: hash::Hash,
}

impl Transaction {
    /// Create a new transaction with the inputs, the outputs and the change.
    /// Doesn't check for correctness of the transaction!
    pub fn new(
        inputs: Vec<input::Input>,
        output: output::Output,
        change: Option<output::Output>,
    ) -> Self {
        // Flatten the transaction into an array such that it can be hashed
        let mut transaction: Vec<u8> = Vec::new();

        // First the inputs
        for input in &inputs {
            let bytes = input.as_bytes();
            transaction.extend(bytes);
        }
        // Then the output
        let bytes = output.as_bytes();
        transaction.extend(bytes);

        // Then the change, if it exists
        if let Some(change) = &change {
            transaction.extend(change.as_bytes());
        }

        // Then hash that
        let id_hash = hash::Hash::create(transaction);

        Self {
            inputs,
            output,
            change,
            id_hash,
        }
    }

    /// Get the complete transaction as flattened bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        // Flatten the transaction into an array such that it can be hashed
        let mut transaction: Vec<u8> = Vec::new();

        // First the inputs
        for input in &self.inputs {
            let bytes = input.as_bytes();
            transaction.extend(bytes);
        }
        // Then the output
        let bytes = self.output.as_bytes();
        transaction.extend(bytes);

        // Then the change, if it exists
        if let Some(change) = &self.change {
            transaction.extend(change.as_bytes());
        }

        // And then the hash
        transaction.extend(self.id_hash.as_bytes());

        transaction
    }

    /// Return all the inputs.
    pub fn get_inputs(&self) -> &Vec<input::Input> {
        &self.inputs
    }

    /// Return the output.
    pub fn get_output(&self) -> &output::Output {
        &self.output
    }

    /// Returns the summed amount of the output and change.
    pub fn get_amount(&self) -> u64 {
        self.output.get_amount()
            + match &self.change {
                Some(change) => change.get_amount(),
                None => 0,
            }
    }
}

use std::fmt;

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Transaction]:{}\n", self.id_hash)?;
        for input in &self.inputs {
            write!(f, "{}\n", input)?;
        }
        if let Some(output) = &self.change {
            write!(f, "{}\n", output)?;
        }
        write!(f, "{}", self.output)
    }
}
