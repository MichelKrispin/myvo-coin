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

    pub fn info(&self) {
        println!(
            "Input {} is owned by {} proofed by signature {} with {}",
            self.input_counter,
            self.output_reference_id,
            self.validator.signature,
            self.validator.public_key,
        )
    }
}
