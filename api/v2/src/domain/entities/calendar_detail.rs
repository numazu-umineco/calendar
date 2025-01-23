use crate::domain::entities::calendar_event::CalendarEvent;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CalendarDetail {
    pub id: String,
    pub name: String,
    pub discarded_at: Option<DateTime<Utc>>,
    pub events: Vec<CalendarEvent>,
}

impl CalendarDetail {
    pub fn new(id: String, name: String) -> CalendarDetail {
        CalendarDetail {
            id,
            name,
            discarded_at: None,
            events: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_calendar_detail() {
        let calendar = CalendarDetail::new("1".to_string(), "Test Calendar".to_string());
        assert_eq!(calendar.id, "1".to_string());
        assert_eq!(calendar.name, "Test Calendar");
        assert!(calendar.discarded_at.is_none());
        assert!(calendar.events.is_empty());
    }

    #[test]
    fn test_discard_calendar_detail() {
        let calendar = CalendarDetail::new("1".to_string(), "Test Calendar".to_string()).discard();
        assert!(calendar.is_discarded());
    }
}
