pub struct PublicKeyHash {
    hash: String,
}

impl PublicKeyHash {
    pub fn create(hash: String) -> Self {
        Self { hash }
    }

    pub fn get(&self) -> &String {
        &self.hash
    }
}
