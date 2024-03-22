
pub mod wallet_struct;

use bdk::Wallet;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();
  println!("Starting server at: http://127.0.0.1:8080");


   
  HttpServer::new(|| {
      App::new()
          .configure(configure_routes)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

