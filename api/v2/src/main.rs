mod domain;
mod infra;
mod use_cases;

use crate::infra::adapters::admin_routes::init as admin_init;
use crate::infra::adapters::public_routes::init as public_init;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(public_init).configure(admin_init))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
