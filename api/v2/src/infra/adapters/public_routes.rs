use crate::infra::exporter::ics::IcsExporter;
use crate::use_cases::calendar_use_case::CalendarUseCase;
use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use std::path::PathBuf;

async fn public_health_alive() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "success" }))
}

async fn public_calendar_all() -> impl Responder {
    let use_case = CalendarUseCase {};
    match use_case.get_all_calendar_details() {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn public_calendar_download(path: web::Path<(String,)>, req: HttpRequest) -> impl Responder {
    let calendar_id = path.into_inner().0;
    let use_case = CalendarUseCase {};
    let exporter = IcsExporter::new();
    match use_case.export_calendar(calendar_id, exporter) {
        Ok(file_path) => {
            let path: PathBuf = file_path.parse().unwrap();
            match NamedFile::open(path) {
                Ok(file) => file.into_response(&req),
                Err(_) => HttpResponse::NotFound().json(json!({ "error": "File not found" })),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn public_calendar_event_recent() -> impl Responder {
    let use_case = CalendarUseCase {};
    match use_case.get_recent_events(10) {
        Ok(json) => {
            println!("{:?}", json);
            HttpResponse::Ok().json(json)
        }

        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/public")
            .route("/health/alive", web::get().to(public_health_alive))
            .route("/calendar/all", web::get().to(public_calendar_all))
            .route(
                "/calendar/download/{id}",
                web::get().to(public_calendar_download),
            )
            .route(
                "/calendar/event/recent",
                web::get().to(public_calendar_event_recent),
            ),
    );
}
