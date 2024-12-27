use api::domain::calendar_detail::CalendarDetail;
use api::domain::calendar_event::CalendarEvent;
use chrono::Utc;

#[test]
fn test_new() {
    let calendar = CalendarDetail::new(1, String::from("Test Calendar"));
    assert_eq!(calendar.id, 1);
    assert_eq!(calendar.summary, "Test Calendar");
    assert!(calendar.discarded_at.is_none());
    assert!(calendar.events.is_empty());
    assert!(calendar.created_at <= Utc::now());
    assert!(calendar.updated_at <= Utc::now());
}

#[test]
fn test_discard() {
    let calendar = CalendarDetail::new(1, String::from("Test Calendar")).discard();
    assert!(calendar.is_discarded());
    assert!(calendar.discarded_at.is_some());
    assert!(calendar.updated_at <= Utc::now());
}

#[test]
fn test_is_discarded() {
    let calendar = CalendarDetail::new(1, String::from("Test Calendar"));
    assert!(!calendar.is_discarded());
    let calendar = calendar.discard();
    assert!(calendar.is_discarded());
}

#[test]
fn test_add_event() {
    let calendar = CalendarDetail::new(1, String::from("Test Calendar"));
    let event = create_event();
    let event_added = calendar.add_event(event);
    assert_eq!(event_added.events.len(), 1);
    assert_eq!(event_added.events[0].id, 1);
    assert_eq!(event_added.events[0].summary, "Test Event");
}

fn create_event() -> CalendarEvent {
    CalendarEvent::new(
        1,
        String::from("Test Event"),
        String::from("Test Description"),
        String::from("Test Location"),
        Utc::now(),
        Utc::now(),
        0.0,
        0.0,
        false,
    )
}
