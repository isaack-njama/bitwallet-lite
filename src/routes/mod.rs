use actix_web::web;

use actix_web::HttpResponse;
use actix_web::Responder;
use bitcoin::util::address::Address;
use bitcoin::secp256k1::{SecretKey, Secp256k1};
use serde_json::json;
use bitcoin::secp256k1::PublicKey as SecpPublicKey;
use bitcoin::PublicKey;
use crate::wallet::WalletInfo;
use crate::wallet::{Wallet, ImportWalletInfo};





// Handler function for creating a new wallet
async fn create_wallet(info: web::Json<WalletInfo>) -> HttpResponse {
   
    match Wallet::new(&info.name) {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(error) => {
            // Return an error response
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create wallet",
                "details": error.to_string(),
            }))
        }
    }
}
 // 
// Handler function for importing an existing wallet
async fn import_wallet(info: web::Json<ImportWalletInfo>) -> HttpResponse {
    let private_key = info.private_key.clone();
    let wallet = Wallet::import(&info.name, &private_key.to_string());
    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn generate_keypair() -> impl Responder {
    // Generate a new random private key
    let secp = Secp256k1::new();
    let private_key_bytes: [u8; 32] = rand::random(); // Generate 32 random bytes
    let private_key = SecretKey::from_slice(&private_key_bytes)
        .expect("Failed to generate a new private key");

   // Derive the public key from the private key
   let public_key = SecpPublicKey::from_secret_key(&secp, &private_key);

   // Convert the public key to the bitcoin::PublicKey type
   let bitcoin_public_key = PublicKey::from_slice(public_key.serialize().as_ref())
       .expect("Failed to convert public key");


    // Derive the address from the public key
    let address = Address::p2pkh(&bitcoin_public_key, bitcoin::Network::Bitcoin);

    // Return the private key, public key, and address
    HttpResponse::Ok().json(json!({
        "private_key": private_key.to_string(),
        "public_key": public_key.to_string(),
        "address": address.to_string(),
    }))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .route("/generate_keypair", web::get().to(generate_keypair))
        .route("/create_wallet", web::post().to(create_wallet))
        .route("/import_wallet", web::post().to(import_wallet))

    );
}
