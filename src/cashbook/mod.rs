mod balance;
mod receipt;
mod wallet;

use crate::blockchain;

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
    /// the public keys in the wallet.
    pub fn get_balance(&self) -> balance::Balance {
        // TODO: Go through the complete blockchain and search
        //       for outputs that belong to the wallets keys
        balance::Balance::empty()
    }
}

use std::fmt;

impl fmt::Display for CashBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            ">>> [CashBook] <<<\n\n{}\n === \n\n{}<<< [CashBook] >>>",
            self.wallet, self.blockchain
        )
    }
}
