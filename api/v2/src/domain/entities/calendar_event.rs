use chrono::{DateTime, Utc};

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
    pub calendar_id: String,
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
        calendar_id: String,
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
            1,
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
        assert_eq!(event.id, 1);
        assert_eq!(event.summary, "Test Event");
        assert_eq!(event.description, "Description");
        assert_eq!(event.location, "Location");
        assert_eq!(event.latitude, 0.0);
        assert_eq!(event.longitude, 0.0);
        assert!(!event.all_day);
        assert_eq!(event.url, None);
        assert_eq!(event.calendar_id, "1".to_string());
    }

    // 他のテストを追加する場合はここに書きます
}
