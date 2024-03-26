use crate::wallet_struct::{ImportWalletInfo, NewAddressInfo, SendBitcoinInfo, WalletInfo, WalletStruct};
use actix_web::web;
use actix_web::HttpResponse;
use serde_json::json;

// Handler function for creating a new wallet
async fn create_wallet(info: web::Json<WalletInfo>) -> HttpResponse {
    let name = info.name.clone();
    let wallet = WalletStruct::create_wallet(&name);

    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": format!("{}", e)
        })),
    }
}

// Handler function for importing an existing wallet
async fn import_wallet(info: web::Json<ImportWalletInfo>) -> HttpResponse {
    let wallet = WalletStruct::import_wallet(&info.phrase);
    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": format!("{}", e)
        })),
    }
}

async fn send_bitcoin(info: web::Json<SendBitcoinInfo>) -> HttpResponse {
    let recipient_address = info.recipient_address.clone();
    let amount = info.amount;
    let wallet = WalletStruct::get_wallet(&info.phrase);
    match wallet {
        Ok(wallet) => {
            let tx_details = WalletStruct::send_bitcoin(&wallet, &recipient_address, amount);
            match tx_details {
                Ok(tx_details) => HttpResponse::Ok().json(json!({
                    "txid": tx_details.txid,
                    "fee": tx_details.fee,
                })),
                Err(e) => HttpResponse::BadRequest().json(json!({
                    "error": format!("{}", e)
                })),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": format!("{}", e)
        })),
    }
}

async fn get_address(info: web::Json<NewAddressInfo>) -> HttpResponse {
    let phrase = info.phrase.clone();
    let wallet = WalletStruct::get_wallet(&phrase);
    match wallet {
        Ok(wallet) => {
            let address = WalletStruct::get_address(&wallet);
            match address {
                Ok(address) => HttpResponse::Ok().json(json!({
                    "address": address
                })),
                Err(e) => HttpResponse::BadRequest().json(json!({
                    "error": format!("{}", e)
                })),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": format!("{}", e)
        })),
    }
}

async fn list_transactions(info: web::Json<NewAddressInfo>) -> HttpResponse {
    let phrase = info.phrase.clone();
    let wallet = WalletStruct::get_wallet(&phrase);
    match wallet {
        Ok(wallet) => {
            let transactions = WalletStruct::get_transactions(&wallet);
            match transactions {
                Ok(transactions) => HttpResponse::Ok().json(json!({
                    "transactions": transactions
                })),
                Err(e) => HttpResponse::BadRequest().json(json!({
                    "error": format!("{}", e)
                })),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": format!("{}", e)
        })),
    }
}

async fn get_balance(info: web::Json<NewAddressInfo>) -> HttpResponse {
    let phrase = info.phrase.clone();
    let wallet = WalletStruct::get_wallet(&phrase);
    match wallet {
        Ok(wallet) => {
            let balance = WalletStruct::get_balance(&wallet);
            match balance {
                Ok(balance) => HttpResponse::Ok().json(json!({
                    "balance": balance
                })),
                Err(e) => HttpResponse::BadRequest().json(json!({
                    "error": format!("{}", e)
                })),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": format!("{}", e)
        })),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/create_wallet", web::post().to(create_wallet))
            .route("/import_wallet", web::post().to(import_wallet))
            .route("/send_bitcoin", web::post().to(send_bitcoin))
            .route("/get_wallet_address", web::post().to(get_address))
            .route("/list_transactions", web::post().to(list_transactions))
            .route("/get_balance", web::post().to(get_balance)),
    );
}
