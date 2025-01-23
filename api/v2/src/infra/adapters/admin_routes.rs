use crate::use_cases::calendar_use_case::CalendarUseCase;
use actix_web::{web, HttpResponse, Responder};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::json;

static CALENDAR_USE_CASE: Lazy<CalendarUseCase> = Lazy::new(|| CalendarUseCase {});

#[derive(Deserialize)]
struct CreateCalendarDetail {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct UpdateCalendarDetail {
    name: String,
}

async fn admin_calendar_details_index() -> impl Responder {
    match CALENDAR_USE_CASE.get_all_calendar_details() {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn admin_calendar_details_create(item: web::Json<CreateCalendarDetail>) -> impl Responder {
    let id = item.id.clone();
    let name = item.name.clone();
    match CALENDAR_USE_CASE.create_calendar_detail(id, name) {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn admin_calendar_details_show(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;
    match CALENDAR_USE_CASE.get_calendar_detail(id) {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn admin_calendar_details_update(
    path: web::Path<(String,)>,
    item: web::Json<UpdateCalendarDetail>,
) -> impl Responder {
    let id = path.into_inner().0;
    let name = item.name.clone();
    match CALENDAR_USE_CASE.update_calendar_detail(id, name) {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn admin_calendar_details_destroy(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;
    match CALENDAR_USE_CASE.delete_calendar(id) {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Calendar deleted" })),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn admin_calendar_events_index() -> impl Responder {
    match CALENDAR_USE_CASE.get_all_events() {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn admin_calendar_events_create() -> impl Responder {
    "admin_calendar_events_create"
}

async fn admin_calendar_events_show(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;
    match CALENDAR_USE_CASE.get_event(id) {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e.to_string() })),
    }
}

async fn admin_calendar_events_update(path: web::Path<(String,)>) -> impl Responder {
    format!("admin_calendar_events_update: {}", path.into_inner().0)
}

async fn admin_calendar_events_destroy(path: web::Path<(String,)>) -> impl Responder {
    format!("admin_calendar_events_destroy: {}", path.into_inner().0)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route(
                "/calendar/details",
                web::get().to(admin_calendar_details_index),
            )
            .route(
                "/calendar/details",
                web::post().to(admin_calendar_details_create),
            )
            .route(
                "/calendar/details/{id}",
                web::get().to(admin_calendar_details_show),
            )
            .route(
                "/calendar/details/{id}",
                web::patch().to(admin_calendar_details_update),
            )
            .route(
                "/calendar/details/{id}",
                web::put().to(admin_calendar_details_update),
            )
            .route(
                "/calendar/details/{id}",
                web::delete().to(admin_calendar_details_destroy),
            )
            .route(
                "/calendar/events",
                web::get().to(admin_calendar_events_index),
            )
            .route(
                "/calendar/events",
                web::post().to(admin_calendar_events_create),
            )
            .route(
                "/calendar/events/{id}",
                web::get().to(admin_calendar_events_show),
            )
            .route(
                "/calendar/events/{id}",
                web::patch().to(admin_calendar_events_update),
            )
            .route(
                "/calendar/events/{id}",
                web::put().to(admin_calendar_events_update),
            )
            .route(
                "/calendar/events/{id}",
                web::delete().to(admin_calendar_events_destroy),
            ),
    );
}
