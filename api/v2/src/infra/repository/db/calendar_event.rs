use std::sync::Arc;

use crate::domain::entities::calendar_event::CalendarEvent;
use crate::infra::schema::calendar_event::CalendarEventSchema;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct CalendarEventRepository {
    conn: Connection,
}

impl CalendarEventRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn all(&self) -> Result<Vec<CalendarEventSchema>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, summary, start_at, end_at, location, description, created_at, updated_at FROM calendar_events",
        )?;
        let rows = stmt.query_map([], |row| Self::map_calendar_event_schema(row))?;

        rows.collect()
    }

    pub fn get_event(&self, id: &str) -> Result<Option<CalendarEventSchema>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, summary, start_at, end_at, location, description, created_at, updated_at FROM calendar_events WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let schema = Self::map_calendar_event_schema(row)?;
            Ok(Some(schema))
        } else {
            Ok(None)
        }
    }

    pub fn create_event(&self, entity: &CalendarEvent) -> Result<usize> {
        let schema = CalendarEventSchema::from_entity(entity.clone());
        self.conn.execute(
            "INSERT INTO calendar_events (id, summary, start_at, end_at, location, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![schema.id, schema.summary, schema.start_at.to_rfc3339(), schema.end_at.to_rfc3339(), schema.location, schema.description, schema.created_at.to_rfc3339(), schema.updated_at.to_rfc3339()],
        )
    }

    pub fn update_event(&self, entity: &CalendarEvent) -> Result<usize> {
        let schema = CalendarEventSchema::from_entity(entity.clone());
        self.conn.execute(
            "UPDATE calendar_events SET summary = ?1, start_at = ?2, end_at = ?3, location = ?4, description = ?5, updated_at = ?6 WHERE id = ?7",
            params![schema.summary, schema.start_at.to_rfc3339(), schema.end_at.to_rfc3339(), schema.location, schema.description, schema.updated_at.to_rfc3339(), schema.id],
        )
    }

    pub fn delete_event(&self, id: &str) -> Result<usize> {
        self.conn
            .execute("DELETE FROM calendar_events WHERE id = ?1", params![id])
    }

    fn map_calendar_event_schema(row: &rusqlite::Row<'_>) -> Result<CalendarEventSchema> {
        let start_at: String = row.get(2)?;
        let end_at: String = row.get(3)?;
        let created_at: String = row.get(6)?;
        let updated_at: String = row.get(7)?;

        Ok(CalendarEventSchema {
            id: row.get(0)?,
            summary: row.get(1)?,
            description: row.get(2)?,
            location: row.get(3)?,
            latitude: row.get(4)?,
            longitude: row.get(5)?,
            start_at: parse_datetime(&start_at)?,
            end_at: parse_datetime(&end_at)?,
            calendar_id: row.get(6)?,
            created_at: parse_datetime(&created_at)?,
            updated_at: parse_datetime(&updated_at)?,
            all_day: row.get(13)?,
            url: row.get(14)?,
        })
    }
}

fn parse_datetime(datetime_str: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(datetime_str)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
        .map(|dt| dt.with_timezone(&Utc))
}
