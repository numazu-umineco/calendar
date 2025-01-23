use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::infra::mappers::calendar_event_mapper::CalendarEventMapper;
use serde_json::json;
use serde_json::Value;

pub struct CalendarDetailMapper;

impl CalendarDetailMapper {
    pub fn to_json(entity: CalendarDetail) -> Value {
        json!({
            "id": entity.id,
            "name": entity.name,
            "events": entity.events.iter().map(|event| CalendarEventMapper::to_json(event.clone())).collect::<Vec<_>>(),
        })
    }
}
