
mod routes;
pub mod wallet_struct;


use actix_cors::Cors;
use actix_web::{App, HttpServer};
use routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();
  

  HttpServer::new(|| {
      let cors = Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header()
                .max_age(3600);

        App::new().wrap(cors).configure(configure_routes)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
