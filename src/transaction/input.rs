use super::validator;

use crate::crypto::hash;

use serde;

const INPUT_LENGTH: usize = hash::HASH_LENGTH + validator::VALIDATOR_LENGTH;

/// The input of a transaction that references the output of
/// another transaction and has to provide some validator
/// to proof ownership of that output.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
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

    pub fn get_output_reference(&self) -> &hash::Hash {
        &self.output_reference
    }

    pub fn get_validator(&self) -> &validator::Validator {
        &self.validator
    }

    pub fn as_bytes(&self) -> [u8; INPUT_LENGTH] {
        let v1 = self.output_reference.as_bytes();
        let v2 = self.validator.as_bytes();
        let whole: Vec<u8> = v1.iter().chain(v2.iter()).map(|v| *v).collect();
        let whole: [u8; INPUT_LENGTH] = whole.try_into().unwrap();
        whole
    }
}

use std::fmt;

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Input]: {}\n< {}\n>",
            self.output_reference, self.validator
        )
    }
}
