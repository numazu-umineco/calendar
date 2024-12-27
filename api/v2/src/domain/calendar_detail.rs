use crate::domain::calendar_event::CalendarEvent;
use chrono::{DateTime, Utc};
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};

#[derive(Debug)]
pub struct CalendarDetail {
    pub id: i32,
    pub name: String,
    pub discarded_at: Option<DateTime<Utc>>,
    pub events: Vec<CalendarEvent>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CalendarDetail {
    pub fn new(id: i32, name: String) -> CalendarDetail {
        let now = Utc::now();
        CalendarDetail {
            id,
            name,
            discarded_at: None,
            events: vec![],
            created_at: now,
            updated_at: now,
        }
    }

    pub fn discard(mut self) -> CalendarDetail {
        let now = Utc::now();
        self.discarded_at = Some(now);
        self.updated_at = now;
        self
    }

    pub fn is_discarded(&self) -> bool {
        self.discarded_at.is_some()
    }

    pub fn add_event(mut self, event: CalendarEvent) -> CalendarDetail {
        self.events.push(event);
        self
    }

    pub fn to_ical(&self) -> String {
        let mut calendar = Calendar::new().name(self.name.clone())
        for event in &self.events {
            calendar.push(event.to_ical())
        }
        calendar.done()
    }
}
