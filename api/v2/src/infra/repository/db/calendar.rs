use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use chrono::{DateTime, Utc};
use icalendar::{Calendar, CalendarDateTime, Class, Event, Property, Todo};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct CalendarDB {
    conn: Connection,
}

impl CalendarDB {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn create_detail(&self, detail: &CalendarDetail) -> Result<usize> {
        let now = Utc::now();

        self.conn.execute(
            "INSERT INTO calendar_details (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![detail.id, detail.name, now.to_rfc3339(), now.to_rfc3339()],
        )
    }

    pub fn update_detail(&self, detail: &CalendarDetail) -> Result<usize> {
        let now = Utc::now();

        self.conn.execute(
            "UPDATE calendar_details SET name = ?1, updated_at = ?2 WHERE id = ?3",
            params![detail.name, now.to_rfc3339(), detail.id],
        )
    }

    pub fn get_detail(&self, id: &str) -> Result<Option<CalendarDetail>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_at, updated_at FROM calendar_details WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let detail = CalendarDetail::new(row.get(0)?, row.get(1)?);
            Ok(Some(detail))
        } else {
            Ok(None)
        }
    }

    pub fn discard_detail(&self, detail: &CalendarDetail) -> Result<usize> {
        let discarded = detail.clone().discard();
        let discarded_at = discarded.discarded_at.unwrap();
        self.conn.execute(
            "UPDATE calendar_details SET discarded_at = ?1 WHERE id = ?2",
            params![discarded_at.to_rfc3339(), discarded.id],
        )
    }
}
