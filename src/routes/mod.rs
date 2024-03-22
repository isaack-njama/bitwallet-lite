
use actix_web::web;
use actix_web::HttpResponse;
use crate::wallet_struct::{WalletStruct, WalletInfo};


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
/* async fn import_wallet(info: web::Json<ImportWalletInfo>) -> HttpResponse {
    let private_key = info.private_key.clone();
    let wallet = Wallet::import(&info.name, &private_key.to_string());
    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
} */





pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
   
        .route("/create_wallet", web::post().to(create_wallet))

    );
}
