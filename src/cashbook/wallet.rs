use crate::crypto;

use std::fs;

pub struct Wallet {
    keypairs: Vec<crypto::keypair::Keypair>,
    wallet_folder: String,
}

impl Wallet {
    /// Open a wallet in the given directory.
    /// The directory should exist and either contain
    /// no files at all or .pk files with existing keys.
    pub fn open(wallet_folder: String) -> Self {
        let keypairs = Wallet::load(&wallet_folder);
        Self {
            keypairs,
            wallet_folder,
        }
    }

    pub fn get_keypairs(&self) -> &Vec<crypto::keypair::Keypair> {
        &self.keypairs
    }

    /// Load up all .pk keypairs inside of the `wallet_folder`.
    fn load(wallet_folder: &String) -> Vec<crypto::keypair::Keypair> {
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
                                let keypair = crypto::keypair::Keypair::load(String::from(path))
                                    .expect("Error loading keypair");
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

    /// Create a new keypair and save it.
    /// Then return the public key as hex.
    pub fn create_keypair(&mut self) -> String {
        // Create a new keypair and generate its hash
        let keypair = crypto::keypair::Keypair::new();
        let public_key_as_hex = keypair.public_key().as_hex();

        // Save it and store it in this wallet
        let filepath = format!("{}/{}.pk", self.wallet_folder, public_key_as_hex);
        keypair.save(filepath);
        self.keypairs.push(keypair);

        public_key_as_hex
    }
}

use std::fmt;

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " --- [Wallet] ('{}') ---\n", &self.wallet_folder)?;
        for keypair in &self.keypairs {
            write!(f, "{}.pk\n", keypair.public_key().as_hex())?;
        }
        Ok(())
    }
}
