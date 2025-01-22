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
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut cal = CalendarDetail::new(
        "fanza-event".to_string(),
        "FANZAイベントカンレンダー".to_string(),
    );
    let now = Utc::now();
    let events = [
        create_event(
            "50％OFFセール第1弾".to_string(),
            "50％OFFセール第1弾".to_string(),
            now,
            now,
            cal.id.clone(),
        ),
        create_event(
            "50％OFFセール第2弾".to_string(),
            "50％OFFセール第2弾".to_string(),
            now,
            now,
            cal.id.clone(),
        ),
        create_event(
            "50％OFFセール第3弾".to_string(),
            "50％OFFセール第3弾".to_string(),
            now,
            now,
            cal.id.clone(),
        ),
    ];
    events.iter().for_each(|e| cal.add_event(e.clone()));

    let db_path = "db/dev.db";
    let calendar_repo =
        CalendarDetailRepository::new(db_path).expect("Failed to initialize database");
    let _ = calendar_repo.create_detail(&cal);

    let event_repo = CalendarEventRepository::new(db_path).expect("Failed to initialize database");
    cal.events.iter().for_each(|e| {
        let res = event_repo.create_event(e);
        println!("{:?}", res);
    });
    Ok(())
}

fn create_event(
    summary: String,
    description: String,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    calendar_id: String,
) -> CalendarEvent {
    let now = Utc::now();
    CalendarEvent {
        id: Uuid::new_v4().to_string(),
        summary,
        description,
        location: "location".to_string(),
        start_at,
        end_at,
        latitude: 0.0,
        longitude: 0.0,
        all_day: false,
        calendar_id,
        url: None,
    }
}

// let mut detail = CalendarDetail::new("numzu-cal4".to_string(), "沼津カレンダー4".to_string());
// let event = CalendarEvent::new(
//     1,
//     "hoge".to_string(),
//     "fuga".to_string(),
//     "location".to_string(),
//     Utc::now(),
//     Utc::now(),
//     0.0,
//     0.0,
//     false,
//     None,
//     detail.id.clone(),
// );
// detail.add_event(event);

// let exporter = IcsExporter::new(detail.clone());
// // let ical = exporter.to_ical();
// // println!("{}", ical);

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
