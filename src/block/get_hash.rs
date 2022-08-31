use crate::crypto::hash;

pub trait GetHash {
    fn hash(&self) -> hash::Hash;
}
