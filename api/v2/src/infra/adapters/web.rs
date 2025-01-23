use actix_web::{web, App, HttpServer, Responder};

use crate::infra::adapters::admin_routes;
use crate::infra::adapters::public_routes;

async fn index() -> impl Responder {
    "Hello, world!"
}
