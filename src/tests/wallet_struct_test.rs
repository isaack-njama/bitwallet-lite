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
}
