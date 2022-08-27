mod input;
mod output;
mod public_key_hash;
mod validator;

pub struct Transaction {
    pub inputs: Vec<input::Input>,
    pub output: output::Output,
    pub change: Option<output::Output>,
    pub id_hash: String,
}

impl Transaction {
    pub fn empty() -> Self {
        Self {
            inputs: vec![
                input::Input::create(
                    String::from("last_output_1"),
                    validator::Validator {
                        signature: String::from("signature_proofing_last_output_1"),
                        public_key: String::from("full_public_key_last_output_1"),
                    },
                    0,
                ),
                input::Input::create(
                    String::from("last_output_2"),
                    validator::Validator {
                        signature: String::from("signature_proofing_last_output_2"),
                        public_key: String::from("full_public_key_last_output_2"),
                    },
                    1,
                ),
            ],
            output: output::Output {
                amount: 5,
                owner: public_key_hash::PublicKeyHash::create(String::from("new_owner")),
            },
            change: None,
            id_hash: String::from("very_unique_hash"),
        }
    }

    pub fn info(&self) {
        for input in &self.inputs {
            input.info();
        }
        self.output.info();
    }
}
