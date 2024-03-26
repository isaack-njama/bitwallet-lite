#[cfg(test)]
mod tests {
    use crate::wallet_struct::WalletStruct;

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
        let mnemonic_phrase = "screen always funny riot garment emerge canvas insane chuckle ice decade cigar";

        // Call the import_wallet function with the sample mnemonic phrase
        let imported_wallet = WalletStruct::import_wallet(mnemonic_phrase).unwrap();

        // Add assertions to check the properties of the imported wallet
        assert_eq!(imported_wallet.name, "Imported Wallet");
        assert!(imported_wallet.address.is_some());
        assert!(imported_wallet.public_key.is_some());
        assert!(imported_wallet.private_key.is_none()); // Assuming the private key is not returned
        assert_eq!(imported_wallet.mnemonic.unwrap(), mnemonic_phrase);
    }
}
