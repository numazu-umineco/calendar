use crate::domain::entities::calendar_event::CalendarEvent;
use serde_json::json;
use serde_json::to_value;
use serde_json::Value;

pub struct CalendarEventMapper;

impl CalendarEventMapper {
    pub fn to_json(entity: CalendarEvent) -> Value {
        json!({
            "id": entity.id,
            "summary": entity.summary,
            "description": entity.description,
            "location": entity.location,
            "start_at": to_value(&entity.start_at).unwrap(), // Modify this line
            "end_at": to_value(&entity.end_at).unwrap(), // Modify this line
            "latitude": entity.latitude,
            "longitude": entity.longitude,
            "all_day": entity.all_day,
            "url": entity.url,
            "calendar_id": entity.calendar_id,
        })
    }
}
