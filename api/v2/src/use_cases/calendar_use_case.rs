use crate::infra::exporter::ics::IcsExporter;
use crate::infra::mappers::calendar_detail_mapper::CalendarDetailMapper;
use crate::infra::mappers::calendar_event_mapper::CalendarEventMapper;
use crate::infra::repository::db::calendar_detail::CalendarDetailRepository;
use crate::infra::repository::db::calendar_event::CalendarEventRepository;
use serde_json::Value;
use uuid::Uuid;

pub struct CalendarUseCase;

impl CalendarUseCase {
    pub fn get_all_calendar_details(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarDetailRepository::new("db/dev.db")?;
        let calendars = repo.kept_all()?;
        let json = calendars
            .iter()
            .map(|calendar| calendar.to_entity())
            .map(|entity| CalendarDetailMapper::to_json(entity))
            .collect::<Vec<_>>();
        Ok(Value::Array(json))
    }

    pub fn export_calendar(
        &self,
        calendar_id: String,
        exporter: IcsExporter,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let repo = CalendarDetailRepository::new("db/dev.db")?;
        let calendar = repo.get_detail_with_events(calendar_id.clone());
        match calendar {
            Ok(Some(calendar)) => {
                let entity = calendar.to_entity();
                let path = format!("tmp/{}-{}.ics", Uuid::new_v4().to_string(), calendar_id);
                exporter.export(entity, &path)?;
                Ok(path)
            }
            Ok(None) => Err("Calendar not found".into()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn get_recent_events(&self, limit: usize) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarEventRepository::new("db/dev.db")?;
        let events = repo.recent(limit)?;
        let json = events
            .iter()
            .map(|event| event.to_entity())
            .map(|entity| CalendarEventMapper::to_json(entity))
            .collect::<Vec<_>>();
        Ok(Value::Array(json))
    }
}
