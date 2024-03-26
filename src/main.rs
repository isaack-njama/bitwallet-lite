
mod routes;
pub mod wallet_struct;

use actix_web::{App, HttpServer};
use routes::configure_routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();
  
   
  HttpServer::new(|| {
      App::new()
          .configure(configure_routes)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

