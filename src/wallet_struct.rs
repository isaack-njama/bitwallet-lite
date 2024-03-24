
use serde::{Deserialize, Serialize};

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bdk::{
  Wallet,
  SyncOptions,
  database::MemoryDatabase,
  electrum_client::Client,
  bitcoin::Amount,
  TransactionDetails,
  Error
};


use std::str::FromStr;
use bdk::wallet::AddressIndex::New;
use bdk::blockchain::ElectrumBlockchain;
use bitcoin::{secp256k1, util::bip32::{ExtendedPrivKey, ExtendedPubKey}, Network};

use bitcoin::util::bip32::ChildNumber;
use bdk::bitcoin::Address as BitcoinAddress;


#[derive(Deserialize, Serialize)]
pub struct WalletStruct  {
  pub name: String,
  pub address: Option<String>,
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  pub mnemonic: Option<String>,
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
        None,
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
      })
  }

    // Function to send Bitcoin to a recipient address
  pub fn send_bitcoin(wallet_data: &WalletStruct, recipient_address: &str, amount: u64) -> Result<TransactionDetails, Error> {
      // Convert the recipient address string to a Bitcoin Address object
      let recipient_address = BitcoinAddress::from_str(recipient_address)
          .map_err(|e| Error::Generic(e.to_string()))?; // Handle potential parsing error
      
      // Instantiate a wallet object from the WalletStruct data
      let wallet = WalletStruct::create_wallet_from_struct(wallet_data)?;
  
      // Send Bitcoin to the recipient address
      let tx_details = wallet.send(vec![(recipient_address.payload.script_pubkey(), Amount::from_sat(amount))], None)?;
  
      Ok(tx_details)
  }

  fn create_wallet_from_struct(wallet_data: &WalletStruct) -> Result<Wallet<MemoryDatabase>, Error> {
    // Extract relevant data from the WalletStruct
    let mnemonic = wallet_data.mnemonic.as_ref().ok_or(Error::Generic("Mnemonic phrase not found".to_string()))?;
    let network = bitcoin::Network::Bitcoin; // Set the Bitcoin network (e.g., Bitcoin mainnet)
    
    // Create a new wallet from the mnemonic
    let wallet = Wallet::new(mnemonic, None,bdk::bitcoin::Network::Testnet, MemoryDatabase::default())?;

    Ok(wallet)
}

  pub fn get_address(wallet: &WalletStruct) -> Result<Option<String>, Error> {
    // Generate a new receiving address
    
    Ok(wallet.address.clone())

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