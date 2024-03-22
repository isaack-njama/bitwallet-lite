use bdk::Wallet;
use bitcoin::Network;
use rand::thread_rng;

pub struct WalletStruct {
  pub name: String,
  pub address: Option<String>, // Add the address field
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  pub mnemonic: Option<String>,
}

impl WalletStruct {
  pub fn new(name: &str) -> Result<Self, String> {
    let wallet = Wallet::new_offline(name, "", Network::Testnet, thread_rng()).unwrap();
    let mnemonic = wallet.mnemonic().unwrap();
    let address = wallet.get_new_address().unwrap();
    let pubkey = wallet.get_descriptor_pk(&address).unwrap();
    let privkey = wallet.get_descriptor_secret(&address).unwrap();

    Ok(Self {
      name: name.to_string(),
      address: address.to_string(), // Assign the address value
      public_key: pubkey.to_string(),
      private_key: privkey.to_string(),
      mnemonic: mnemonic.to_string(),
    })
  }

 /*  pub fn import(name: &str, private_key: &str) -> Result<Self, String> {
    Ok(Self {
      name: name.to_string(),
      address: String::new(), // Initialize the address field
      public_key: String::new(),
      private_key: private_key.to_string(),
      mnemonic: String::new(),
    })
  } */
}