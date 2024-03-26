#[cfg(test)]
mod tests {
    use crate::wallet_struct::WalletStruct;
    use bdk::{database::MemoryDatabase, Wallet};
    use bip39::Seed;
    use bitcoin::{
        secp256k1,
        util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey},
        Network,
    };

    #[test]
    fn test_generate_mnemonic() {
        let mnemonic = WalletStruct::generate_mnemonic().unwrap();
        // Assert that the generated mnemonic is not empty
        assert!(!mnemonic.phrase().is_empty());
    }

    #[test]
    fn test_create_wallet() {
        // Test creating a wallet
        let wallet = WalletStruct::create_wallet("Test Wallet").unwrap();
        assert_eq!(wallet.name, "Test Wallet");
    }

    #[test]
    fn test_import_wallet() {
        // Define a sample mnemonic phrase
        let mnemonic_phrase =
            "screen always funny riot garment emerge canvas insane chuckle ice decade cigar";

        // Call the import_wallet function with the sample mnemonic phrase
        let imported_wallet = WalletStruct::import_wallet(mnemonic_phrase).unwrap();

        // Add assertions to check the properties of the imported wallet
        assert_eq!(imported_wallet.name, "Imported Wallet");
        assert!(imported_wallet.address.is_some());
        assert!(imported_wallet.public_key.is_some());
        assert!(imported_wallet.private_key.is_none()); // Assuming the private key is not returned
        assert_eq!(imported_wallet.mnemonic.unwrap(), mnemonic_phrase);
    }

    #[test]
    fn test_get_address() {
        let mnemonic = WalletStruct::generate_mnemonic().unwrap();
        let seed = Seed::new(&mnemonic, "");
        // Derive the master extended private key
        let master_extended_private_key =
            ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes()).unwrap();
        let extended_private_key = master_extended_private_key
            .derive_priv(&secp256k1::Secp256k1::new(), &[ChildNumber::from_hardened_idx(84 + 0).unwrap()])
            .unwrap();


        // Derive the extended public key from the extended private key
        let extended_public_key =
            ExtendedPubKey::from_private(&secp256k1::Secp256k1::new(), &extended_private_key);

        let descriptor= format!("wpkh({})", extended_public_key.to_string());

        let database = MemoryDatabase::default();

        let wallet =
            Wallet::new(&descriptor, None, bdk::bitcoin::Network::Testnet, database).unwrap();

        let address = match WalletStruct::get_address(&wallet) {
            Ok(address) => address,
            Err(err) => {
                panic!("Failed to get address: {:?}", err);
            }
        };

        assert!(address.starts_with("tb1"));
    }

    #[test]
    fn test_get_transactions() {
        let mnemonic = WalletStruct::generate_mnemonic().unwrap();
        let seed = Seed::new(&mnemonic, "");
        // Derive the master extended private key
        let master_extended_private_key =
            ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes()).unwrap();
        let extended_private_key = master_extended_private_key
            .derive_priv(&secp256k1::Secp256k1::new(), &[ChildNumber::from_hardened_idx(84 + 0).unwrap()])
            .unwrap();


        // Derive the extended public key from the extended private key
        let extended_public_key =
            ExtendedPubKey::from_private(&secp256k1::Secp256k1::new(), &extended_private_key);

        let descriptor= format!("wpkh({})", extended_public_key.to_string());

        // Create a mock Wallet<MemoryDatabase> object
        let database = MemoryDatabase::default();
        let wallet = Wallet::new(&descriptor, None, bdk::bitcoin::Network::Testnet, database).unwrap();

        // Call the get_transactions function with the mock Wallet<MemoryDatabase>
        let transactions = match WalletStruct::get_transactions(&wallet) {
            Ok(transactions) => transactions,
            Err(err) => {
                panic!("Failed to get transactions: {:?}", err);
            }
        };

        // Add assertions to check the properties of the returned transactions
        assert_eq!(transactions.len(), 0); // Assert that initially there are no transactions
        // Add more assertions as needed
    }
}
