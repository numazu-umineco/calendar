// use std::fs::File;
// use std::io::Write;
// use std::sync::Arc;

//use crate::domain::entities;
use crate::domain::entities::calendar_detail::CalendarDetail;
//use crate::domain::entities::calendar_event::CalendarEvent;
use crate::infra::repository::db::calendar_event::CalendarEventRepository;
use crate::infra::schema::calendar_detail::CalendarDetailSchema;
use chrono::{DateTime, Utc};
//use icalendar::{Calendar, CalendarDateTime, Class, Event, Property, Todo};
//use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct CalendarDetailRepository {
    conn: Connection,
}

impl CalendarDetailRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn all(&self) -> Result<Vec<CalendarDetailSchema>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_at, updated_at, discarded_at FROM calendars WHERE discarded_at IS NULL",
        )?;
        let rows = stmt.query_map([], |row| Self::map_calendar_detail_schema(row))?;

        rows.collect()
    }

    pub fn kept_all(&self) -> Result<Vec<CalendarDetailSchema>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_at, updated_at, discarded_at FROM calendars WHERE discarded_at IS NULL",
        )?;
        let rows = stmt.query_map([], |row| Self::map_calendar_detail_schema(row))?;

        rows.collect()
    }

    pub fn get_detail(&self, id: &str) -> Result<Option<CalendarDetailSchema>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_at, updated_at, discarded_at FROM calendar_details WHERE id = ?1",
        )?;
        let mut rows: rusqlite::Rows<'_> = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let schema = Self::map_calendar_detail_schema(row)?;
            Ok(Some(schema))
        } else {
            Ok(None)
        }
    }

    pub fn create_detail(&self, entity: &CalendarDetail) -> Result<usize> {
        let schema = CalendarDetailSchema::from_entity(entity);
        let _ = self.conn.execute(
            "INSERT INTO calendar_details (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![schema.id, schema.name, schema.created_at.to_rfc3339(), schema.updated_at.to_rfc3339()],
        );
        Ok(0)
    }

    pub fn create_detail_with_events(
        &self,
        entity: &CalendarDetail,
        event_repository: CalendarEventRepository,
    ) -> Result<usize> {
        let schema = CalendarDetailSchema::from_entity(entity);
        let _= self.conn.execute(
            "INSERT INTO calendar_details (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![schema.id, schema.name, schema.created_at.to_rfc3339(), schema.updated_at.to_rfc3339()],
        );

        let _ = entity
            .events
            .iter()
            .map(|e| event_repository.create_event(e));
        Ok(0)
    }

    pub fn update_detail(&self, entity: &CalendarDetail) -> Result<usize> {
        let schema = CalendarDetailSchema::from_entity(entity);
        self.conn.execute(
            "UPDATE calendar_details SET name = ?1, updated_at = ?2 WHERE id = ?3",
            params![schema.name, schema.updated_at.to_rfc3339(), schema.id],
        )
    }

    pub fn discard_detail(&self, entity: &CalendarDetail) -> Result<usize> {
        let discarded_at = entity.discarded_at.unwrap_or_else(Utc::now).to_rfc3339();
        self.conn.execute(
            "UPDATE calendar_details SET discarded_at = ?1, updated_at = ?2 WHERE id = ?3",
            params![discarded_at, discarded_at, entity.id],
        )
    }

    fn map_calendar_detail_schema(row: &rusqlite::Row<'_>) -> Result<CalendarDetailSchema> {
        let created_at: String = row.get(2)?;
        let updated_at: String = row.get(3)?;
        let discarded_at: Option<String> = row.get(4)?;

        Ok(CalendarDetailSchema {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: parse_datetime(&created_at)?,
            updated_at: parse_datetime(&updated_at)?,
            discarded_at: discarded_at.map(|d| parse_datetime(&d)).transpose()?,
            events: vec![],
        })
    }
}

fn parse_datetime(datetime_str: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(datetime_str)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
        .map(|dt| dt.with_timezone(&Utc))
}
