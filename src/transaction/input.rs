use super::validator;

pub struct Input {
    output_reference_id: String,
    validator: validator::Validator,
    input_counter: u8,
}

impl Input {
    pub fn create(
        output_reference_id: String,
        validator: validator::Validator,
        input_counter: u8,
    ) -> Self {
        Self {
            output_reference_id,
            validator,
            input_counter,
        }
    }
}

use std::fmt;

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Input]:{} - {}\n  <{}>",
            self.input_counter, self.output_reference_id, self.validator
        )
    }
}
