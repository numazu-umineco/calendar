use actix_web::{web, Responder};

async fn public_health_alive() -> impl Responder {
    "public_health_alive"
}

async fn public_calendar_all() -> impl Responder {
    "public_calendar_all"
}

async fn public_calendar_download(path: web::Path<(String,)>) -> impl Responder {
    format!("public_calendar_download: {}", path.into_inner().0)
}

async fn public_calendar_event_recent() -> impl Responder {
    "public_calendar_event_recent"
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
