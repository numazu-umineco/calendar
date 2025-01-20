use actix_web::{web, Responder};

async fn admin_calendar_details_index() -> impl Responder {
    "admin_calendar_details_index"
}

async fn admin_calendar_details_create() -> impl Responder {
    "admin_calendar_details_create"
}

async fn admin_calendar_details_show(path: web::Path<(String,)>) -> impl Responder {
    format!("admin_calendar_details_show: {}", path.into_inner().0)
}

async fn admin_calendar_details_update(path: web::Path<(String,)>) -> impl Responder {
    format!("admin_calendar_details_update: {}", path.into_inner().0)
}

async fn admin_calendar_details_destroy(path: web::Path<(String,)>) -> impl Responder {
    format!("admin_calendar_details_destroy: {}", path.into_inner().0)
}

async fn admin_calendar_events_index() -> impl Responder {
    "admin_calendar_events_index"
}

async fn admin_calendar_events_create() -> impl Responder {
    "admin_calendar_events_create"
}

async fn admin_calendar_events_show(path: web::Path<(String,)>) -> impl Responder {
    format!("admin_calendar_events_show: {}", path.into_inner().0)
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
