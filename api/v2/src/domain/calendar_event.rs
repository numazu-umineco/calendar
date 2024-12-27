use chrono::{DateTime, Utc};
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};

#[derive(Debug)]
pub struct CalendarEvent {
    pub id: i32,
    pub summary: String,
    pub description: String,
    pub location: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub all_day: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CalendarEvent {
    pub fn new(
        id: i32,
        summary: String,
        description: String,
        location: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        latitude: f64,
        longitude: f64,
        all_day: bool,
    ) -> CalendarEvent {
        let now = Utc::now();
        CalendarEvent {
            id,
            summary,
            description,
            location,
            start,
            end,
            latitude,
            longitude,
            all_day,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn to_ical(&self) -> Event {
        Event::new()
            .summary(&self.summary.clone())
            .description(&self.description.clone())
            .starts(CalendarDateTime::ne(&self.start))
            .ends(CalendarDateTime::ne(&self.end))
            .class(Class::Public)
            .location(&self.location.clone())
            .done()
    }

    pub fn geo(&self) -> String {
        format!("{};{}", self.latitude, self.longitude)
    }
}
