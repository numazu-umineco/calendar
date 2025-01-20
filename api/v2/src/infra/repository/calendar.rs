use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use chrono::{DateTime, Utc};
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct CalendarRepository {
    conn: Connection,
}

impl CalendarRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn create_event(&self, event: &CalendarEvent) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO calendar_events (summary, description, location, start_at, end_at, latitude, longitude, all_day, url) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                event.summary,
                event.description,
                event.location,
                event.start_at.to_rfc3339(),
                event.end_at.to_rfc3339(),
                event.latitude,
                event.longitude,
                event.all_day,
                event.url,
            ],
        )
    }

    pub fn read_event(&self, id: i32) -> Result<CalendarEvent> {
        self.conn.query_row(
            "SELECT id, summary, description, location, start_at, end_at, latitude, longitude, all_day, url FROM calendar_events WHERE id = ?1",
            params![id],
            |row| {
                Ok(CalendarEvent {
                    id: row.get(0)?,
                    summary: row.get(1)?,
                    description: row.get(2)?,
                    location: row.get(3)?,
                    start_at: row.get::<_, String>(4)?.parse::<DateTime<Utc>>().unwrap(),
                    end_at: row.get::<_, String>(5)?.parse::<DateTime<Utc>>().unwrap(),
                    latitude: row.get(6)?,
                    longitude: row.get(7)?,
                    all_day: row.get(8)?,
                    url: row.get(9)?,
                })
            },
        )
    }

    pub fn update_event(&self, event: &CalendarEvent) -> Result<usize> {
        self.conn.execute(
            "UPDATE calendar_events SET summary = ?1, description = ?2, location = ?3, start_at = ?4, end_at = ?5, latitude = ?6, longitude = ?7, all_day = ?8, url = ?9 WHERE id = ?10",
            params![
                event.summary,
                event.description,
                event.location,
                event.start_at.to_rfc3339(),
                event.end_at.to_rfc3339(),
                event.latitude,
                event.longitude,
                event.all_day,
                event.url,
                event.id,
            ],
        )
    }

    pub fn delete_event(&self, id: i32) -> Result<usize> {
        self.conn
            .execute("DELETE FROM calendar_events WHERE id = ?1", params![id])
    }

    pub fn to_ical(&self) -> Result<Calendar> {
        let mut stmt = self.conn.prepare("SELECT id, summary, description, location, start_at, end_at, latitude, longitude, all_day, url FROM calendar_events")?;
        let events = stmt.query_map(params![], |row| {
            Ok(CalendarEvent {
                id: row.get(0)?,
                summary: row.get(1)?,
                description: row.get(2)?,
                location: row.get(3)?,
                start_at: row.get::<_, String>(4)?.parse::<DateTime<Utc>>().unwrap(),
                end_at: row.get::<_, String>(5)?.parse::<DateTime<Utc>>().unwrap(),
                latitude: row.get(6)?,
                longitude: row.get(7)?,
                all_day: row.get(8)?,
                url: row.get(9)?,
            })
        })?;

        let mut calendar = Calendar::new();
        for event in events {
            let event = event?;
            calendar.push(Self::event_to_ical(&event));
        }
        Ok(calendar)
    }

    fn event_to_ical(event: &CalendarEvent) -> Event {
        Event::new()
            .summary(&event.summary)
            .description(&event.description)
            .starts(CalendarDateTime::from(event.start_at))
            .ends(CalendarDateTime::from(event.end_at))
            .class(Class::Public)
            .location(&event.location)
            .done()
    }
}
