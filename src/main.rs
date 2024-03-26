#[cfg(test)]
#[path = "./tests/wallet_struct_test.rs"]
mod wallet_struct_test;

mod routes;
pub mod wallet_struct;

use actix_web::{App, HttpServer};
use routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("Starting server at: http://127.0.0.1:8080");

    HttpServer::new(|| App::new().configure(configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
