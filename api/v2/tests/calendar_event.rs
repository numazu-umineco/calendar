use api::domain::calendar_event::CalendarEvent;
use chrono::Utc;

#[test]
fn test_new_calendar_event() {
    let event = CalendarEvent::new(
        1,
        String::from("Test Event"),
        String::from("Test Description"),
        String::from("Test Location"),
        Utc::now(),
        Utc::now(),
        0.0,
        0.0,
        false,
    );
    assert_eq!(event.id, 1);
    assert_eq!(event.summary, "Test Event");
    assert_eq!(event.description, "Test Description");
    assert_eq!(event.location, "Test Location");
    assert_eq!(event.latitute, 0.0);
    assert_eq!(event.longitude, 0.0);
    assert_eq!(event.all_day, false);
    assert!(event.created_at <= Utc::now());
    assert!(event.updated_at <= Utc::now());
}

fn test_geo() {
    let latitude = 135.0;
    let longitude = 66.0;
    let event = CalendarEvent::new(
        1,
        String::from("Test Event"),
        String::from("Test Description"),
        String::from("Test Location"),
        Utc::now(),
        Utc::now(),
        latitude,
        longitude,
        false,
    );
    let expects = format!("{};{}", latitude, longitude);
    assert_eq!(event.geo(), expects);
}
