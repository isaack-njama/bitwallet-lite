
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
    fn test_get_wallet() {
        let phrase =
            "screen always funny riot garment emerge canvas insane chuckle ice decade cigar";

        let wallet_result = WalletStruct::get_wallet(phrase);

        match wallet_result {
            Ok(wallet) => {
                assert_eq!(wallet.network(), bdk::bitcoin::Network::Testnet);
            }
            Err(err) => {
                panic!("Failed to create wallet: {:?}", err);
            }
        }
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
            .derive_priv(
                &secp256k1::Secp256k1::new(),
                &[ChildNumber::from_hardened_idx(84 + 0).unwrap()],
            )
            .unwrap();

        // Derive the extended public key from the extended private key
        let extended_public_key =
            ExtendedPubKey::from_private(&secp256k1::Secp256k1::new(), &extended_private_key);

        let descriptor = format!("wpkh({})", extended_public_key.to_string());

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
        let master_extended_private_key =
            ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes()).unwrap();
        let extended_private_key = master_extended_private_key
            .derive_priv(
                &secp256k1::Secp256k1::new(),
                &[ChildNumber::from_hardened_idx(84 + 0).unwrap()],
            )
            .unwrap();

        let extended_public_key =
            ExtendedPubKey::from_private(&secp256k1::Secp256k1::new(), &extended_private_key);

        let descriptor = format!("wpkh({})", extended_public_key.to_string());

        let database = MemoryDatabase::default();
        let wallet =
            Wallet::new(&descriptor, None, bdk::bitcoin::Network::Testnet, database).unwrap();

        let transactions = match WalletStruct::get_transactions(&wallet) {
            Ok(transactions) => transactions,
            Err(err) => {
                panic!("Failed to get transactions: {:?}", err);
            }
        };

        assert_eq!(transactions.len(), 0); // Assert that initially there are no transactions
    }

    #[test]
    fn test_get_balance() {
        let mnemonic = WalletStruct::generate_mnemonic().unwrap();
        let seed = Seed::new(&mnemonic, "");
        let master_extended_private_key =
            ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes()).unwrap();
        let extended_private_key = master_extended_private_key
            .derive_priv(
                &secp256k1::Secp256k1::new(),
                &[ChildNumber::from_hardened_idx(84 + 0).unwrap()],
            )
            .unwrap();

        let extended_public_key =
            ExtendedPubKey::from_private(&secp256k1::Secp256k1::new(), &extended_private_key);

        let descriptor = format!("wpkh({})", extended_public_key.to_string());

        let database = MemoryDatabase::default();
        let wallet =
            Wallet::new(&descriptor, None, bdk::bitcoin::Network::Testnet, database).unwrap();

        // Call the get_balance function with the mock Wallet<MemoryDatabase>
        let balance = match WalletStruct::get_balance(&wallet) {
            Ok(balance) => balance,
            Err(err) => {
                panic!("Failed to get balance: {:?}", err);
            }
        };

        assert_eq!(balance.immature, 0); // Assert that immature balance is 0
    }

    #[test]
    fn test_send_bitcoin() {
        let mnemonic = WalletStruct::generate_mnemonic().unwrap();
        let seed = Seed::new(&mnemonic, "");
        let master_extended_private_key =
            ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes()).unwrap();
        let extended_private_key = master_extended_private_key
            .derive_priv(
                &secp256k1::Secp256k1::new(),
                &[ChildNumber::from_hardened_idx(84 + 0).unwrap()],
            )
            .unwrap();

        let extended_public_key =
            ExtendedPubKey::from_private(&secp256k1::Secp256k1::new(), &extended_private_key);

        let descriptor = format!("wpkh({})", extended_public_key.to_string());

        let database = MemoryDatabase::default();
        let wallet = Wallet::new(&descriptor, None, bdk::bitcoin::Network::Testnet, database).unwrap();

        let recipient_address = "tb1q2d4s9mk2qjdcylhvy42tzj32wr3pxjl4a48n09";
        let amount = 34; // Amount in satoshis

        let transaction_result = WalletStruct::send_bitcoin(&wallet, recipient_address, amount);

        match transaction_result {
            Ok(details) => {
                assert!(details.fee < Some(1000)); // Ensure that the fee is reasonable
            }
            Err(err) => {
                println!("Failed to send Bitcoin: {:?}", err);
                assert!(err.to_string().contains("bech32 address encoding error"));
            }
        }
    }
}
