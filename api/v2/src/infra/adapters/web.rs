use actix_web::{web, App, HttpServer, Responder};

use crate::infra::adapters::admin_routes;
use crate::infra::adapters::public_routes;

async fn index() -> impl Responder {
    "Hello, world!"
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .configure(public_routes::init)
            .configure(admin_routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
