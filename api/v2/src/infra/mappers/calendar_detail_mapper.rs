use crate::domain::entities::calendar_detail::CalendarDetail;
use serde_json::json;
use serde_json::Value;

pub struct CalendarDetailMapper;

impl CalendarDetailMapper {
    pub fn to_json(entity: CalendarDetail) -> Value {
        json!({
            "id": entity.id,
            "name": entity.name,
        })
    }
}
