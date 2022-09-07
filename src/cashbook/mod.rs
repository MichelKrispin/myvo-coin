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

    /// Return all the outputs that belong to
    /// the public keys in the wallet and that haven't
    /// been used.
    pub fn get_balance(&self) -> balance::Balance {
        // Go through the complete blockchain and search
        // for outputs that belong to the wallets keys
        let mut receipts: Vec<receipt::Receipt> = vec![];
        for keypair in self.wallet.get_keypairs() {
            // First hash the public key
            let public_key_hash = {
                let public_key = keypair.public_key();
                crypto::hash::Hash::create(public_key.as_hex())
            };
            // Then search for the transaction output
            // and continue if it has already been used
            let output = match self.blockchain.get_valid_output(public_key_hash) {
                Some(output) => output,
                None => continue,
            };

            // Reload the keypair for the receipt
            let copied_keypair = {
                let keypair_path = format!("keys/{}.pk", keypair.public_key().as_hex());
                crypto::keypair::Keypair::load(keypair_path)
            };
            let receipt = receipt::Receipt::create(output.get_amount(), copied_keypair);
            // Then save it
            receipts.push(receipt);
        }
        balance::Balance::create(receipts)
    }
}

use std::fmt;

impl fmt::Display for CashBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let balance = self.get_balance();
        write!(
            f,
            ">>> [CashBook] <<<\n\n{}\n ===\n{}\n<<< [CashBook] >>>",
            self.wallet, balance
        )
    }
}
