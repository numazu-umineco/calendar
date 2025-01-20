mod domain;
mod infra;

use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::infra::exporter::ics::IcsExporter;
use crate::infra::repository::calendar::CalendarRepository;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use infra::repository;
use std::fs::File;
use std::io::Write;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut calendar = CalendarDetail::new(1, "沼津イベント".to_string());

    let event = CalendarEvent::new(
        1,
        "沼津イベント".to_string(),
        "沼津でイベントを開催します".to_string(),
        "沼津市".to_string(),
        Utc::now(),
        Utc::now(),
        35.1,
        138.9,
        false,
        None,
    );

    calendar.add_event(event);

    let repo = CalendarRepository::new(calendar);
    let _ = repo.to_ical();

    Ok(())
}
