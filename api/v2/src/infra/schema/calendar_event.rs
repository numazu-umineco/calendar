use crate::domain::entities::calendar_event::CalendarEvent;
use chrono::{DateTime, Utc};

pub struct CalendarEventSchema {
    pub id: i32,
    pub summary: String,
    pub description: String,
    pub location: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub all_day: bool,
    pub calendar_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub url: Option<String>,
}

impl CalendarEventSchema {
    pub fn from_entity(entity: CalendarEvent) -> Self {
        let now = Utc::now();
        Self {
            id: entity.id,
            summary: entity.summary,
            description: entity.description,
            location: entity.location,
            start_at: entity.start_at,
            end_at: entity.end_at,
            latitude: entity.latitude,
            longitude: entity.longitude,
            all_day: entity.all_day,
            calendar_id: entity.calendar_id,
            created_at: now,
            updated_at: now,
            url: entity.url,
        }
    }

    pub fn to_entity(&self) -> CalendarEvent {
        CalendarEvent {
            id: self.id,
            summary: self.summary.clone(),
            description: self.description.clone(),
            location: self.location.clone(),
            start_at: self.start_at,
            end_at: self.end_at,
            latitude: self.latitude,
            longitude: self.longitude,
            all_day: self.all_day,

            url: self.url.clone(),
            calendar_id: self.calendar_id.clone(),
        }
    }
}
