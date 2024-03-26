#[cfg(test)]
mod tests {
    use crate::wallet_struct::WalletStruct;

    #[test]
    fn test_generate_mnemonic() {
        let mnemonic = WalletStruct::generate_mnemonic().unwrap();
        // Assert that the generated mnemonic is not empty
        assert!(!mnemonic.phrase().is_empty());
    }
}
