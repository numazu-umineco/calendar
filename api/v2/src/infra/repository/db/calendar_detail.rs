// use std::fs::File;
// use std::.io::Write;
// use std::sync::Arc;

//use crate::domain::entities;
use crate::{
    domain::entities::calendar_detail::CalendarDetail,
    infra::schema::calendar_event::CalendarEventSchema,
};
//use crate::domain::entities::calendar_event::CalendarEvent;
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

    pub fn kept_all(&self) -> Result<Vec<CalendarDetailSchema>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_at, updated_at, discarded_at FROM calendar_details WHERE discarded_at IS NULL",
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

    pub fn get_detail_with_events(&self, id: String) -> Result<Option<CalendarDetailSchema>> {
        let mut stmt = self.conn.prepare(
            "
            SELECT
                d.id,
                d.name,
                d.created_at,
                d.updated_at,
                d.discarded_at,
                e.id,
                e.summary,
                e.description,
                e.location,
                e.latitude,
                e.longitude,
                e.start_at,
                e.end_at,
                e.discarded_at,
                e.calendar_detail_id,
                e.last_modified_user,
                e.created_at,
                e.updated_at,
                e.all_day,
                e.url
            FROM calendar_details d
            LEFT JOIN calendar_events e ON d.id = e.calendar_detail_id
            WHERE d.id = ?1",
        )?;
        let mut rows: rusqlite::Rows<'_> = stmt.query(params![id])?;

        let mut events = vec![];
        let mut calendar_detail: Option<CalendarDetailSchema> = None;

        while let Some(row) = rows.next()? {
            if calendar_detail.is_none() {
                calendar_detail = Some(CalendarDetailSchema {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: parse_datetime(&row.get::<_, String>(2)?)?,
                    updated_at: parse_datetime(&row.get::<_, String>(3)?)?,
                    discarded_at: row
                        .get::<_, Option<String>>(4)?
                        .map(|d| parse_datetime(&d))
                        .transpose()?,
                    events: vec![],
                });
            }

            if let Some(event_id) = row.get::<_, Option<String>>(5)? {
                events.push(CalendarEventSchema {
                    id: event_id,
                    summary: row.get(6)?,
                    description: row.get(7)?,
                    location: row.get(8)?,
                    latitude: row.get(9)?,
                    longitude: row.get(10)?,
                    start_at: parse_datetime(&row.get::<_, String>(11)?)?,
                    end_at: parse_datetime(&row.get::<_, String>(12)?)?,
                    calendar_id: row.get(14)?,
                    created_at: parse_datetime(&row.get::<_, String>(16)?)?,
                    updated_at: parse_datetime(&row.get::<_, String>(17)?)?,
                    all_day: row.get(18)?,
                    url: row.get(19)?,
                });
            }
        }

        if let Some(mut detail) = calendar_detail {
            detail.events = events;
            Ok(Some(detail))
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

    pub fn update_detail(&self, entity: &CalendarDetail) -> Result<usize> {
        let schema = CalendarDetailSchema::from_entity(entity);
        self.conn.execute(
            "UPDATE calendar_details SET name = ?1, updated_at = ?2 WHERE id = ?3",
            params![schema.name, schema.updated_at.to_rfc3339(), schema.id],
        )
    }

    pub fn delete(&self, id: String) -> Result<usize> {
        self.conn
            .execute("DELETE FROM calendar_details WHERE id = ?1", params![id])
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
