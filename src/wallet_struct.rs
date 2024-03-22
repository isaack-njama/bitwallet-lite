
use serde::{Deserialize, Serialize};


use bip39::Mnemonic;
use bdk::{
  Wallet,
  SyncOptions,
  database::MemoryDatabase,
  electrum_client::Client,
};
use  rand::{thread_rng, RngCore};
use bdk::wallet::AddressIndex::New;
use bdk::blockchain::ElectrumBlockchain;





#[derive(Deserialize, Serialize)]
pub struct WalletStruct {
  pub name: String,
  pub address:  String,
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  pub mnemonic: Option<String>,
}

impl WalletStruct {

  pub fn generate_mnemonic() -> Result<Mnemonic, Box<dyn std::error::Error>> {
      let mut seed = [0; 32];
      thread_rng().fill_bytes(&mut seed);
      Ok(Mnemonic::from_entropy(&seed)?)
  }

  /* pub fn derive_keys(mnemonic: &Mnemonic) -> Result<(ExtendedPrivKey, ExtendedPubKey), Box<dyn std::error::Error>> {
      let seed = Seed::new(&mnemonic, "");
      let master_extended_private_key = ExtendedPrivKey::new_master( bdk::bitcoin::Network::Testnet, &seed.as_bytes())?;
      let extended_private_key = master_extended_private_key.derive_priv(&KeySource::from_normal_idx(0, 0))?;
      let extended_public_key = ExtendedPubKey::from_private(&bdk::bitcoin::Network::Testnet, &extended_private_key);
      Ok((extended_private_key, extended_public_key))
  } */

  pub fn create_wallet(name: &str) -> Result<WalletStruct, Box<dyn std::error::Error>> {
    //let mnemonic = generate_mnemonic()?;
    //let (extended_private_key, extended_public_key) = derive_keys(&mnemonic)?;

    let descriptor = format!("wpkh({})", name.to_string());

    let database = MemoryDatabase::default();
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        &descriptor,
        None,
        bdk::bitcoin::Network::Testnet,
        database,
    )?;

    wallet.sync(&blockchain, SyncOptions::default())?;

    Ok(WalletStruct {
        name: name.to_string(),
        address: wallet.get_address(New).unwrap().to_string(),
        public_key: None,
        private_key: None,
       mnemonic: None,
    })
  }
  
}
    


pub struct ImportWalletInfo {
  pub name: String,
  pub private_key: String,
}

pub struct WalletInfo {
  pub name: String,
}