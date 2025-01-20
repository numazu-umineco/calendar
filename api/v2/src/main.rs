mod domain;
mod infra {
    pub mod adapters;
    pub mod repository; // 追加
}

use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::infra::repository::calendar::CalendarRepository;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infra::adapters::web::run().await
}
