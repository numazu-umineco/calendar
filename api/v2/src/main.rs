mod domain;
mod infra;

use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::infra::exporter::ics::IcsExporter;
use crate::infra::repository::db::calendar_detail::CalendarDetailRepository;
use crate::infra::repository::db::calendar_event::CalendarEventRepository;
use actix_web::{web, App, HttpServer};
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut detail = CalendarDetail::new("numzu-cal4".to_string(), "沼津カレンダー4".to_string());
    let event = CalendarEvent::new(
        1,
        "hoge".to_string(),
        "fuga".to_string(),
        "location".to_string(),
        Utc::now(),
        Utc::now(),
        0.0,
        0.0,
        false,
        None,
        detail.id.clone(),
    );
    detail.add_event(event);

    // let exporter = IcsExporter::new(detail.clone());
    // // let ical = exporter.to_ical();
    // // println!("{}", ical);

    let db_path = "db/dev.db";
    let calendar_db =
        CalendarDetailRepository::new(db_path).expect("Failed to initialize database");
    // let mut cal = match calendar_db.get_detail(&detail.id) {
    //     Ok(Some(cal)) => cal,
    //     Ok(None) => match calendar_db.create_detail(&detail) {
    //         Ok(_) => {
    //             println!("CalendarDetail created successfully");
    //             detail
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to create CalendarDetail: {}", e);
    //             return Ok(());
    //         }
    //     },
    //     Err(e) => {
    //         eprintln!("Failed to get CalendarDetail: {}", e);
    //         return Ok(());
    //     }
    // };
    let mut cal = calendar_db.get_detail(&detail.id);
    println!("{:?}", cal);

    // calendar_db.discard_detail(&cal);

    Ok(())
}
