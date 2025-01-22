use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CalendarEvent {
    pub id: String,
    pub summary: String,
    pub description: String,
    pub location: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub all_day: bool,
    pub url: Option<String>,
    pub calendar_id: String,
}

impl CalendarEvent {
    pub fn new(
        id: Option<String>,
        summary: String,
        description: String,
        location: String,
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        latitude: f64,
        longitude: f64,
        all_day: bool,
        url: Option<String>,
        calendar_id: String,
    ) -> CalendarEvent {
        CalendarEvent {
            id: id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            summary,
            description,
            location,
            start_at,
            end_at,
            latitude,
            longitude,
            all_day,
            url,
            calendar_id,
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
            Some("1".to_string()),
            "Test Event".to_string(),
            "Description".to_string(),
            "Location".to_string(),
            Utc::now(),
            Utc::now(),
            0.0,
            0.0,
            false,
            None,
            "1".to_string(),
        );
        assert_eq!(event.id, "1".to_string());
        assert_eq!(event.summary, "Test Event".to_string());
        assert_eq!(event.description, "Description".to_string());
        assert_eq!(event.location, "Location".to_string());
        assert_eq!(event.latitude, 0.0);
        assert_eq!(event.longitude, 0.0);
        assert!(!event.all_day);
        assert_eq!(event.url, None);
        assert_eq!(event.calendar_id, "1".to_string());
    }

    #[test]
    fn test_new_calendar_event_with_generated_id() {
        let event = CalendarEvent::new(
            None,
            "summary".to_string(),
            "description".to_string(),
            "Location".to_string(),
            Utc::now(),
            Utc::now(),
            0.0,
            0.0,
            false,
            None,
            "1".to_string(),
        );
        assert!(!event.id.is_empty());
        assert_eq!(event.summary, "summary".to_string());
        assert_eq!(event.description, "description".to_string());
        assert_eq!(event.location, "Location".to_string());
        assert_eq!(event.latitude, 0.0);
        assert_eq!(event.longitude, 0.0);
        assert!(!event.all_day);
        assert_eq!(event.url, None);
        assert_eq!(event.calendar_id, "1".to_string());
    }

    // 他のテストを追加する場合はここに書きます
}
