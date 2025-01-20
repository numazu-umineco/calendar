use crate::domain::entities::calendar_event::CalendarEvent;
use chrono::{DateTime, Utc};
use icalendar::{Calendar, CalendarDateTime, Class, Event, EventLike, Property, Todo};

#[derive(Debug)]
pub struct CalendarDetail {
    pub id: i32,
    pub name: String,
    pub discarded_at: Option<DateTime<Utc>>,
    pub events: Vec<CalendarEvent>,
}

impl CalendarDetail {
    pub fn new(id: i32, name: String) -> CalendarDetail {
        CalendarDetail {
            id,
            name,
            discarded_at: None,
            events: vec![],
        }
    }

    pub fn discard(mut self) -> CalendarDetail {
        let now = Utc::now();
        self.discarded_at = Some(now);
        self
    }

    pub fn is_discarded(&self) -> bool {
        self.discarded_at.is_some()
    }

    pub fn add_event(&mut self, event: CalendarEvent) {
        self.events.push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::calendar_event::CalendarEvent;
    use chrono::Utc;

    #[test]
    fn test_new_calendar_detail() {
        let calendar = CalendarDetail::new(1, "Test Calendar".to_string());
        assert_eq!(calendar.id, 1);
        assert_eq!(calendar.name, "Test Calendar");
        assert!(calendar.discarded_at.is_none());
        assert!(calendar.events.is_empty());
    }

    #[test]
    fn test_discard_calendar_detail() {
        let calendar = CalendarDetail::new(1, "Test Calendar".to_string()).discard();
        assert!(calendar.is_discarded());
    }

    #[test]
    fn test_add_event() {
        let mut calendar = CalendarDetail::new(1, "Test Calendar".to_string());
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
        calendar.add_event(event);
        assert_eq!(calendar.events.len(), 1);
        assert_eq!(calendar.events[0].summary, "Test Event");
    }

    // 他のテストを追加する場合はここに書きます
}
