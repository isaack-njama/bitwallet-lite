
use actix_web::web;
use actix_web::HttpResponse;
use crate::wallet_struct::{WalletInfo,WalletStruct, ImportWalletInfo, SendBitcoinInfo};


// Handler function for creating a new wallet
async fn create_wallet(info: web::Json<WalletInfo> ) -> HttpResponse {

    let name = info.name.clone();
    let wallet = WalletStruct::create_wallet(&name);


    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }

}
 
// Handler function for importing an existing wallet
 async fn import_wallet(info: web::Json<ImportWalletInfo>) -> HttpResponse {
   
    let wallet = WalletStruct::import_wallet(&info.phrase);
    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
} 

async fn send_bitcoin(info: web::Json<SendBitcoinInfo>) -> HttpResponse {
    let recipient_address = info.recipient_address.clone();
    let amount = info.amount;
    let wallet = WalletStruct::import_wallet(&info.phrase);
    match wallet {
        Ok(wallet) => {
            let tx_details = WalletStruct::send_bitcoin(&wallet, &recipient_address, amount);
            match tx_details {
                Ok(tx_details) => HttpResponse::Ok().json(tx_details),
                Err(e) => HttpResponse::BadRequest().body(e.to_string()),
            }
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn get_address(info: web::Json<WalletInfo>) -> HttpResponse {
    let name = info.name.clone();
    let wallet = WalletStruct::import_wallet(&name);
    match wallet {
        Ok(wallet) => {
            let address = WalletStruct::get_address(&wallet);
            match address {
                Ok(address) => HttpResponse::Ok().json(address),
                Err(e) => HttpResponse::BadRequest().body(e.to_string()),
            }
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}





pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
   
        .route("/create_wallet", web::post().to(create_wallet))
        .route("/import_wallet", web::post().to(import_wallet))
        .route("/send_bitcoin", web::post().to(send_bitcoin))
        .route("/generate_new_address", web::post().to(get_address))

    );
}
