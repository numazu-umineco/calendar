mod domain;
mod infra;
mod use_cases;

use crate::infra::adapters::public_routes::init;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(init))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
