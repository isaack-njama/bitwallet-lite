use bdk::Wallet;
use bitcoin::Network;
use rand::thread_rng;

pub struct WalletStruct {
    pub name: String,
}

impl WalletStruct {
    pub fn new(name: &str) -> Result<Self, String> {

      let wallet = Wallet::new_offline(name, "", Network::Testnet, thread_rng()).unwrap();

      // Get the mnemonic phrase
      let mnemonic = wallet.mnemonic().unwrap();
  
      // Get the first address
      let address = wallet.get_new_address().unwrap();
   
      // Get the public key associated with the first address
      let pubkey = wallet.get_descriptor_pk(&address).unwrap();
    
      // Get the private key associated with the first address
      let privkey = wallet.get_descriptor_secret(&address).unwrap();

      Ok(Self {
          name: name.to_string(),
          address: address.to_string(),
          public_key: pubkey.to_string(),
          private_key: privkey.to_string(),
          mnemonic: mnemonic.to_string(),

      })
     
    }

    pub fn import(name: &str, private_key: &str) -> Result<Self, String> {
        Ok(Self {
            name: name.to_string(),
        })
    }
}