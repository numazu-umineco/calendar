use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::infra::exporter::ics::IcsExporter;
use crate::infra::mappers::calendar_detail_mapper::CalendarDetailMapper;
use crate::infra::mappers::calendar_event_mapper::CalendarEventMapper;
use crate::infra::repository::db::calendar_detail::CalendarDetailRepository;
use crate::infra::repository::db::calendar_event::CalendarEventRepository;
use crate::use_cases::interfaces::CalendarEventParams;
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

    pub fn get_calendar_detail(&self, id: String) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarDetailRepository::new("db/dev.db")?;
        match repo.get_detail_with_events(id) {
            Ok(Some(calendar)) => {
                let entity = calendar.to_entity();
                let json = CalendarDetailMapper::to_json(entity);
                Ok(json)
            }
            Ok(None) => Err("Calendar not found".into()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn create_calendar_detail(
        &self,
        id: String,
        name: String,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarDetailRepository::new("db/dev.db")?;
        let entity = CalendarDetail::new(id, name);
        let _ = repo.create_detail(&entity)?;
        let json = CalendarDetailMapper::to_json(entity);
        Ok(json)
    }

    pub fn update_calendar_detail(
        &self,
        id: String,
        name: String,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarDetailRepository::new("db/dev.db")?;
        let mut entity = match repo.get_detail(&id) {
            Ok(Some(schema)) => schema.to_entity(),
            Ok(None) => return Err("Calendar not found".into()),
            Err(e) => return Err(Box::new(e)),
        };
        entity.name = name;
        let _ = repo.update_detail(&entity)?;
        let json = CalendarDetailMapper::to_json(entity);
        Ok(json)
    }

    pub fn delete_calendar(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let repo = CalendarDetailRepository::new("db/dev.db")?;
        match repo.delete(id) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn get_all_events(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarEventRepository::new("db/dev.db")?;
        let events = repo.all()?;
        let json = events
            .iter()
            .map(|event| CalendarEventMapper::to_json(event.to_entity()))
            .collect::<Vec<_>>();
        Ok(Value::Array(json))
    }

    pub fn get_event(&self, id: String) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarEventRepository::new("db/dev.db")?;
        match repo.get_event(&id) {
            Ok(Some(event)) => {
                let entity = event.to_entity();
                let json = CalendarEventMapper::to_json(entity);
                Ok(json)
            }
            Ok(None) => Err("Event not found".into()),
            Err(e) => Err(Box::new(e)),
        }
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
            .map(|event| CalendarEventMapper::to_json(event.to_entity()))
            .collect::<Vec<_>>();
        Ok(Value::Array(json))
    }

    pub fn create_calendar_event(
        &self,
        params: CalendarEventParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarEventRepository::new("db/dev.db")?;
        let entity = CalendarEvent::new(
            Some(params.id), // Wrap params.id in Some
            params.summary,
            params.description,
            params.location,
            params.end_at,
            params.start_at,
            params.latitude,
            params.longitude,
            params.all_day,
            params.url,
            params.calendar_id,
        );
        let _ = repo.create_event(&entity)?;
        let json = CalendarEventMapper::to_json(entity);
        Ok(json)
    }

    pub fn update_event(
        &self,
        id: String,
        params: CalendarEventParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let repo = CalendarEventRepository::new("db/dev.db")?;
        let mut entity = match repo.get_event(&id) {
            Ok(Some(event)) => event.to_entity(),
            Ok(None) => return Err("Event not found".into()),
            Err(e) => return Err(Box::new(e)),
        };
        entity.summary = params.summary;
        entity.description = params.description;
        entity.location = params.location;
        entity.end_at = params.end_at;
        entity.start_at = params.start_at;
        entity.latitude = params.latitude;
        entity.longitude = params.longitude;
        entity.all_day = params.all_day;
        entity.url = params.url;
        entity.calendar_id = params.calendar_id;
        let _ = repo.update_event(&entity)?;
        let json = CalendarEventMapper::to_json(entity);
        Ok(json)
    }

    pub fn delete_event(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let repo = CalendarEventRepository::new("db/dev.db")?;
        match repo.delete_event(&id) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
