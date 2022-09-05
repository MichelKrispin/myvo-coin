use crate::crypto::keypair;

use std::fs;

pub struct Wallet {
    keypairs: Vec<keypair::Keypair>,
    wallet_folder: String,
}

impl Wallet {
    /// Open a wallet in the given directory.
    /// The directory should exist and either contain
    /// no files at all or .pub files with existing keys.
    pub fn open(wallet_folder: String) -> Self {
        let keypairs = Wallet::load(&wallet_folder);
        Self {
            keypairs,
            wallet_folder,
        }
    }

    /// Load up all .pub keypairs inside of the `wallet_folder`.
    fn load(wallet_folder: &String) -> Vec<keypair::Keypair> {
        let mut keypairs = vec![];
        // Open up the directory, if it exists.
        let entries = fs::read_dir(wallet_folder).expect("Could not read the wallet folder");
        // Then loop through each entry in the directory
        for entry in entries {
            if let Ok(entry) = entry {
                // Get the file type and only continue, if it is not a folder
                if let Ok(file_type) = entry.file_type() {
                    if !file_type.is_dir() {
                        // Then get the filename to check for .pk files
                        let os_string = entry.file_name();
                        let file_name = os_string
                            .to_str()
                            .expect("Filename contains unknown character");
                        if file_name.ends_with(".pk") {
                            // If one is found, load it up
                            let path_buffer = entry.path();
                            let path = path_buffer.as_path().to_str();
                            if let Some(path) = path {
                                let keypair = keypair::Keypair::load(String::from(path));
                                keypairs.push(keypair);
                                println!("[Wallet] Loaded {:?}", entry.path());
                            }
                        }
                    }
                }
            }
        }

        keypairs
    }

    /// Save all keypairs held by this wallet.
    fn save(&self) {
        for keypair in &self.keypairs {
            keypair.save(keypair.public_key().as_hex() + ".pk");
        }
    }
}

use std::fmt;

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " --- [Wallet] ('{}') ---\n", &self.wallet_folder)?;
        for keypair in &self.keypairs {
            write!(f, "{}.pub\n", keypair.public_key().as_hex())?;
        }
        Ok(())
    }
}
