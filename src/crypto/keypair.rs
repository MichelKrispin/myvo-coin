use super::private_key;
use super::public_key;

pub struct Keypair {
    private_key: private_key::PrivateKey,
    public_key: public_key::PublicKey,
}
