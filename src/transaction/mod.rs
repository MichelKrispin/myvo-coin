pub mod creation;
pub mod input;
pub mod output;
pub mod validator;

use crate::crypto::hash;

/// A transaction that maps one or more inputs to one output
/// and one optional change output.
/// It is uniquely defined by the hash of all of its input values
/// serialized into a long list of bytes.
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
        // TODO: Don't fake that hash.
        // A fake hash
        let id_hash = hash::Hash::create(String::from("serialization of the previous stuff"));

        Self {
            inputs,
            output,
            change,
            id_hash,
        }
    }

    pub fn get_inputs(&self) -> &Vec<input::Input> {
        &self.inputs
    }

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
