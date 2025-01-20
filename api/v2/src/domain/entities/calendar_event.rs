use chrono::{DateTime, Utc};
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};

#[derive(Debug, Clone)]
pub struct CalendarEvent {
    pub id: i32,
    pub summary: String,
    pub description: String,
    pub location: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub all_day: bool,
    pub url: Option<String>,
}

impl CalendarEvent {
    pub fn new(
        id: i32,
        summary: String,
        description: String,
        location: String,
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        latitude: f64,
        longitude: f64,
        all_day: bool,
        url: Option<String>,
    ) -> CalendarEvent {
        CalendarEvent {
            id,
            summary,
            description,
            location,
            start_at,
            end_at,
            latitude,
            longitude,
            all_day,
            url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_new_calendar_event() {
        let event = CalendarEvent::new(
            1,
            "Test Event".to_string(),
            "Description".to_string(),
            "Location".to_string(),
            Utc::now(),
            Utc::now(),
            0.0,
            0.0,
            false,
            None,
        );
        assert_eq!(event.id, 1);
        assert_eq!(event.summary, "Test Event");
        assert_eq!(event.description, "Description");
        assert_eq!(event.location, "Location");
        assert_eq!(event.latitude, 0.0);
        assert_eq!(event.longitude, 0.0);
        assert!(!event.all_day);
        assert!(event.url.is_none());
    }

    // 他のテストを追加する場合はここに書きます
}
