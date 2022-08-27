use super::public_key_hash;

pub struct Output {
    pub amount: u64,
    pub owner: public_key_hash::PublicKeyHash,
}

impl Output {
    pub fn info(&self) {
        println!(
            "The new owner is {} and owns {}",
            self.amount,
            self.owner.get()
        )
    }
}
