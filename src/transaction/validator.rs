pub struct Validator {
    pub signature: String,
    pub public_key: String,
}

use std::fmt;

impl fmt::Display for Validator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Validator] {}:{}", self.signature, self.public_key,)
    }
}
