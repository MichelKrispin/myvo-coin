mod balance;
mod receipt;
mod wallet;

use crate::blockchain;
use crate::crypto;

pub struct CashBook {
    wallet: wallet::Wallet,
    blockchain: blockchain::BlockChain,
}

impl CashBook {
    /// Open up a cashbook that stores the owned keypairs in the
    /// `wallet_folder`. The `blockchain` is the chain of interest.
    pub fn open(wallet_folder: String, blockchain: blockchain::BlockChain) -> Self {
        let wallet = wallet::Wallet::open(wallet_folder);
        Self { wallet, blockchain }
    }

    /// Replace the blockchain with a new blockchain if needed.
    pub fn update_blockchain(&mut self, blockchain: blockchain::BlockChain) {
        self.blockchain = blockchain;
    }

    /// Return all the outputs that belong to
    /// the public keys in the wallet and that haven't
    /// been used.
    pub fn get_balance(&self) -> balance::Balance {
        // Go through the complete blockchain and search
        // for outputs that belong to the wallets keys
        let mut receipts: Vec<receipt::Receipt> = vec![];
        let mut unused: Vec<String> = vec![];
        for keypair in self.wallet.get_keypairs() {
            // First get the public key and hash it
            let public_key_as_hex = {
                let public_key = keypair.public_key();
                public_key.as_hex()
            };
            let public_key_hash = crypto::hash::Hash::create(&public_key_as_hex);

            // Then search for the transaction output
            // and continue if it has already been used
            let output = match self.blockchain.get_valid_output(&public_key_hash) {
                Ok(output) => output,
                Err(error) => match error {
                    blockchain::InvalidOutput::NotFound => {
                        unused.push(public_key_as_hex);
                        continue;
                    }
                    blockchain::InvalidOutput::AlreadyUsed => continue,
                },
            };

            // Reload the keypair for the receipt
            let copied_keypair = {
                let keypair_path = format!("keys/{}.pk", keypair.public_key().as_hex());
                crypto::keypair::Keypair::load(keypair_path).expect("Error loading keypair")
            };
            let receipt = receipt::Receipt::create(output.get_amount(), copied_keypair);
            // Then save it
            receipts.push(receipt);
        }
        balance::Balance::create(receipts, unused)
    }

    /// Create a new keypair that will be saved in the wallet
    /// and return the public key pair as hex that should be used
    /// for the transaction output.
    pub fn create_keypair(&mut self) -> String {
        self.wallet.create_keypair()
    }
}

use std::fmt;

impl fmt::Display for CashBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let balance = self.get_balance();
        write!(
            f,
            ">>> [CashBook] <<<\n\n{}\n\n{}\n<<< [CashBook] >>>",
            self.wallet, balance
        )
    }
}
