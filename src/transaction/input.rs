use super::validator;

use crate::crypto::hash;

/// The input of a transaction that references the output of
/// another transaction and has to provide some validator
/// to proof ownership of that output.
pub struct Input {
    // TODO: Probably better if the correct block is also known.
    /// The hash that corresponds to the referred output.
    output_reference: hash::Hash,

    /// The validator to proof the ownership of the referred output.
    validator: validator::Validator,
}

impl Input {
    /// Create a new hash with the given output hash.
    /// Doesn't check whether the validator actually corresponds to the output.
    pub fn create(output_reference: hash::Hash, validator: validator::Validator) -> Self {
        Self {
            output_reference,
            validator,
        }
    }
}

use std::fmt;

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Input]: {}\n  <{}>",
            self.output_reference, self.validator
        )
    }
}
