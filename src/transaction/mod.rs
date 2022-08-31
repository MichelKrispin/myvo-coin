pub mod creation;
mod input;
mod output;
mod validator;

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
    pub fn empty() -> Self {
        // Two dummy inputs.
        let inputs = vec![
            /*
            input::Input::create(
                hash::Hash::create(String::from("last_output_1")),
                validator::Validator {
                    signature: String::from("signature_proofing_last_output_1"),
                    public_key: String::from("full_public_key_last_output_1"),
                },
            ),
            input::Input::create(
                hash::Hash::create(String::from("last_output_2")),
                validator::Validator {
                    signature: String::from("signature_proofing_last_output_2"),
                    public_key: String::from("full_public_key_last_output_2"),
                },
            ),
            */
        ];
        // A dummy output
        let output = output::Output::create(5, hash::Hash::create(String::from("new_owner")));
        // No change
        let change: Option<output::Output> = None;
        // A fake hash
        let id_hash = hash::Hash::create(String::from("serialization of the previous stuff"));

        Self {
            inputs,
            output,
            change,
            id_hash,
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
