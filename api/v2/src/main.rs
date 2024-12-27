mod domain;

use crate::domain::calendar_detail::CalendarDetail;
use crate::domain::calendar_event::CalendarEvent;
use chrono::{DateTime, Utc};

fn main() {
    let calendar = CalendarDetail::new(1, "沼津イベント".to_string());
    println!("{:?}", calendar.is_discarded());

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
    );
    let event_added = calendar.add_event(event);
    println!("{:?}", event_added);
}
