use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::infra::schema::calendar_event::CalendarEventSchema;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CalendarDetailSchema {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub discarded_at: Option<DateTime<Utc>>,
    pub events: Vec<CalendarEventSchema>,
}

impl CalendarDetailSchema {
    pub fn from_entity(entity: &CalendarDetail) -> Self {
        let now = Utc::now();
        Self {
            id: entity.id.clone(),
            name: entity.name.clone(),
            created_at: now,
            updated_at: now,
            discarded_at: entity.discarded_at,
            events: entity
                .events
                .iter()
                .map(|event| CalendarEventSchema::from_entity(event.clone()))
                .collect(),
        }
    }

    pub fn to_entity(&self) -> CalendarDetail {
        CalendarDetail {
            id: self.id.clone(),
            name: self.name.clone(),
            discarded_at: self.discarded_at,
            events: vec![],
        }
    }
}
