

use serde::{Deserialize, Serialize};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bdk::{
   blockchain::ElectrumBlockchain, database::{ BatchDatabase, MemoryDatabase}, electrum_client::Client, wallet::AddressIndex, Error, FeeRate, SignOptions, SyncOptions, TransactionDetails, Wallet
};

use std::str::FromStr;
use bdk::wallet::AddressIndex::New;
use bbdk::blockchain::electrum;
use bitcoin::{secp256k1, util::bip32::{ExtendedPrivKey, ExtendedPubKey}, Network};

use bitcoin::util::bip32::ChildNumber;
use bdk::bitcoin::Address as BitcoinAddress;

use bitcoin::secp256k1::Secp256k1;

#[derive(Deserialize, Serialize)]
pub struct WalletStruct  {
  pub name: String,
  pub address: Option<String>,
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  pub mnemonic: Option<String>,
  pub balance: Option<WalletBalance>,
}

impl WalletStruct {

  pub fn generate_mnemonic() -> Result<Mnemonic, Box<dyn std::error::Error>> {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    Ok(mnemonic)
  }
  
  pub fn create_wallet(name: &str) -> Result<WalletStruct, Box<dyn std::error::Error>> {
  
    let mnemonic = WalletStruct::generate_mnemonic().unwrap();
    let seed =  Seed::new(&mnemonic, "");
    // Derive the master extended private key
    let master_extended_private_key = ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes())?;
    let extended_private_key = master_extended_private_key.derive_priv(&secp256k1::Secp256k1::new(), &[
      ChildNumber::from_hardened_idx(84 + 0)?,
    ])?;

    // Derive the extended public key from the extended private key
    let extended_public_key = ExtendedPubKey::from_private(&secp256k1::Secp256k1::new(), &extended_private_key);  
    let derivation_path = "m/84'/1'/0'";
    let _descriptor_string = format!("wpkh([{}]{})", derivation_path, extended_public_key);
     
  
    let descriptor = format!("wpkh({})", extended_public_key.to_string());

    let database = MemoryDatabase::default();
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        &descriptor,
        Some(&descriptor),
        bdk::bitcoin::Network::Testnet,
        database,
    )?;

    wallet.sync(&blockchain, SyncOptions::default())?;




    Ok(WalletStruct {
        name: name.to_string(),
        address: Some(wallet.get_address(New).unwrap().to_string()),
        public_key: Some(extended_public_key.to_string()),
        private_key: Some(extended_private_key.to_string()),
       mnemonic: Some(mnemonic.phrase().to_string()),
        balance: Some(WalletStruct::get_balance(&wallet).unwrap()),
    })
  }


  pub fn import_wallet(mnemonic_words: &str) -> Result<WalletStruct, Box<dyn std::error::Error>> {
    // Parse mnemonic phrase
    let mnemonic = Mnemonic::from_phrase(mnemonic_words, Language::English)?;

    // Derive keys from mnemonic
    let seed = Seed::new(&mnemonic, "");
    let master_extended_private_key = ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes())?;
    let extended_public_key = ExtendedPubKey::from_private(&secp256k1::Secp256k1::new(), &master_extended_private_key);

    // Construct descriptor
    let descriptor = format!("wpkh({})", extended_public_key);

    // Initialize wallet
    let database = MemoryDatabase::default();
    let wallet = Wallet::new(
        &descriptor,
        Some(&descriptor),
        bdk::bitcoin::Network::Testnet,
        database,
    )?;

      // Synchronize wallet
      wallet.sync(
        &ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002")?),
        SyncOptions::default(),
     )?;

      Ok(WalletStruct {
        name: "Imported Wallet".to_string(),
        address: Some(wallet.get_address(New).unwrap().to_string()),
        public_key: Some(extended_public_key.to_string()),
        private_key: None,
        mnemonic: Some(mnemonic.phrase().to_string()),
        balance: Some(WalletStruct::get_balance(&wallet).unwrap()),
      })
  }

    // Function to send Bitcoin to a recipient address
  pub fn send_bitcoin<D: BatchDatabase>(wallet: &Wallet<D>, recipient_address: &str, amount: u64) -> Result<TransactionDetails, Error> {
      // Convert the recipient address string to a Bitcoin Address object
      let recipient_address = BitcoinAddress::from_str(recipient_address)
          .map_err(|e| Error::Generic(e.to_string()))?; // Handle potential parsing error
      
      // Instantiate a wallet object from the WalletStruct data
      //let wallet = WalletStruct::create_wallet_from_struct(wallet_data)?;
      let client = Client::new("ssl://electrum.blockstream.info:60002")?;
      let blockchain = ElectrumBlockchain::from(client);

      wallet.sync(&blockchain, SyncOptions::default())?;
  
      
      let mut tx_builder =  wallet.build_tx();

          tx_builder
              .add_recipient(recipient_address.payload.script_pubkey(), amount)
              .enable_rbf()
              //.do_not_spend_change()
              .fee_rate(FeeRate::from_sat_per_vb(0.00001));

      let (psbt, tx_details) = tx_builder.finish()?;
      
      wallet.sign(&mut psbt, SignOptions::default())?;
      
      // Broadcast the transaction
      ElectrumBlockchain::from(client).broadcast(&psbt.extract_tx())?;
  
     Ok(tx_details)
    
  }


  pub fn get_address(wallet: &Wallet<MemoryDatabase>) -> Result<String, Error> {
    // Generate a new receiving address
    let address = wallet.get_address(AddressIndex::New)?;
    Ok(address.to_string())

  }


  pub fn get_wallet(phrase: &str) -> Result<Wallet<MemoryDatabase>, Error> {
    // Parse the mnemonic phrase
    let mnemonic = Mnemonic::from_phrase(phrase, Language::English)
        .map_err(|e| Error::Generic(format!("Error parsing mnemonic phrase: {}", e)))?;
   
    let seed = Seed::new(&mnemonic, "");
    let master_extended_private_key = bitcoin::util::bip32::ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes())
    .map_err(|e| Error::Generic(format!("Error creating master extended private key: {}", e)))?;

    let extended_public_key = ExtendedPubKey::from_private(
        &secp256k1::Secp256k1::new(),
      &master_extended_private_key);
    // Construct descriptor
    let descriptor = format!("wpkh({})", extended_public_key);


    // Create a wallet from the given mnemonic phrase
    let wallet = Wallet::new(
       &descriptor,
        None, // Passphrase if one was used during wallet creation
        bdk::bitcoin::Network::Testnet,  // Specify the network
        <_>::default(), // Use the default database type
      )?;
      
      Ok(wallet)
  }


  pub fn get_transactions(wallet: &Wallet<MemoryDatabase>) -> Result<Vec<TransactionDetails>, Error> {
    // Get the list of transactions from the wallet
    let transactions = wallet.list_transactions(true)?;
    Ok(transactions)
  }

  pub fn get_balance(wallet: &Wallet<MemoryDatabase>) -> Result<WalletBalance, Error> {
    // Get the balance of the wallet
    let balance = wallet.get_balance()?;
    Ok(WalletBalance {
        immature: balance.immature,
        trusted_pending: balance.trusted_pending,
        confirmed: balance.confirmed,
        untrusted_pending: balance.untrusted_pending,
    })
  }

  pub fn get_wallet_by_address(address: &str) -> Result<WalletStruct, Error> {
    // Initialize the Electrum client
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);


    // Construct the descriptor
    let descriptor = WalletStruct::generate_descriptor().unwrap();

    // Initialize the wallet
    let database = MemoryDatabase::default();
    let wallet = Wallet::new(
        &descriptor,
  
        None,
        bdk::bitcoin::Network::Testnet,
        database,

    )?;

    // Synchronize the wallet
    wallet.sync(&blockchain, SyncOptions::default())?;

    Ok(WalletStruct {
        name: "Imported Wallet".to_string(),
        address: Some(wallet.get_address(New).unwrap().to_string()),
        public_key: None,
        private_key: None,
        mnemonic: None,
        balance: Some(WalletStruct::get_balance(&wallet).unwrap()),
    })
  }

  

  fn generate_descriptor() -> Result<String, Box<dyn std::error::Error>>{

    let mnemonic = WalletStruct::generate_mnemonic().unwrap();

     print!("Mnemonic: {:?}", mnemonic.phrase());
    let seed = Seed::new(&mnemonic, "");
    // Derive the master extended private key
    let master_extended_private_key = ExtendedPrivKey::new_master(Network::Testnet, &seed.as_bytes())?;
    let extended_private_key = master_extended_private_key.derive_priv(
        &Secp256k1::new(),
        &[
            ChildNumber::from_hardened_idx(84 + 0)?,
        ],
    )?;

    // Derive the extended public key from the extended private key
    let extended_public_key = ExtendedPubKey::from_private(&Secp256k1::new(), &extended_private_key);
    let derivation_path = "m/84'/1'/0'";
    let descriptor_string = format!("wpkh([{}]{})", derivation_path, extended_public_key);

    Ok(descriptor_string)
   
}
 
}


    


#[derive(Serialize, Deserialize)]
pub struct ImportWalletInfo {
  pub phrase: String,
}
#[derive(Serialize, Deserialize)]
pub struct WalletInfo {
  pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct SendBitcoinInfo {
  pub phrase: String,
  pub recipient_address: String,
  pub amount: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NewAddressInfo {
  pub phrase: String,
}

#[derive(Serialize, Deserialize)]
pub struct WalletBalance {
  pub immature: u64,
  pub trusted_pending: u64,
  pub confirmed: u64,
  pub untrusted_pending : u64,
}

#[derive(Serialize, Deserialize)]
pub struct WalletAddress {
  pub address: String,
}

