mod domain;
mod infra;

use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::infra::exporter::ics::IcsExporter;
use crate::infra::repository::calendar::CalendarRepository;
use chrono::{DateTime, Utc};
use infra::repository;
use std::fs::File;
use std::io::Write;

fn main() {
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
    let event1 = CalendarEvent::new(
        1,
        "野獣先輩".to_string(),
        "お　ま　た　せ、アイスティーしかなかったんだけどいいかな？".to_string(),
        "下北沢".to_string(),
        Utc::now(),
        Utc::now(),
        35.1,
        138.9,
        false,
        None,
    );
    let event2 = CalendarEvent::new(
        1,
        "tnok".to_string(),
        "おいゴルァ! 降りろ! 概要書いてんのかコラ!".to_string(),
        "下北沢".to_string(),
        Utc::now(),
        Utc::now(),
        35.1,
        138.9,
        false,
        None,
    );

    calendar.add_event(event);
    calendar.add_event(event1);
    calendar.add_event(event2);

    let repo = CalendarRepository::new(calendar);
    let ical = repo.to_ical();
    let exporter = IcsExporter::new();
    let _ = exporter.execute(ical, "calendar.ics");
}
