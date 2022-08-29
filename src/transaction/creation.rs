pub struct Creation {
    pub output: output::Output,
    pub id_hash: String,
}

use std::fmt;

impl fmt::Display for Creation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Creation]:{}, {}", self.id_hash, self.output,)
    }
}
