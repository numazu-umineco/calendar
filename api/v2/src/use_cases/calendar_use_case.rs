use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::infra::exporter::ics::IcsExporter;
use crate::infra::mappers::calendar_detail_mapper::CalendarDetailMapper;
use crate::infra::mappers::calendar_event_mapper::CalendarEventMapper;
use crate::infra::repository::db::calendar_detail::CalendarDetailRepository;
use crate::infra::repository::db::calendar_event::CalendarEventRepository;
use crate::use_cases::interfaces::CalendarEventParams;
use dotenv::dotenv;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub struct CalendarUseCase {
    pub calendar_detail_repo: CalendarDetailRepository,
    pub calendar_event_repo: CalendarEventRepository,
}

unsafe impl Sync for CalendarUseCase {}

impl CalendarUseCase {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok(); // Load environment variables from .env file
        let database_url = env::var("DATABASE_URL")?;
        let calendar_detail_repo = CalendarDetailRepository::new(&database_url)?;
        let calendar_event_repo = CalendarEventRepository::new(&database_url)?;
        Ok(Self {
            calendar_detail_repo,
            calendar_event_repo,
        })
    }

    pub fn get_all_calendar_details(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let calendars = self.calendar_detail_repo.kept_all()?;
        let json = calendars
            .iter()
            .map(|calendar| calendar.to_entity())
            .map(|entity| CalendarDetailMapper::to_json(entity))
            .collect::<Vec<_>>();
        Ok(Value::Array(json))
    }

    pub fn get_calendar_detail(&self, id: String) -> Result<Value, Box<dyn std::error::Error>> {
        match self.calendar_detail_repo.get_detail_with_events(id) {
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
        let entity = CalendarDetail::new(id, name);
        let _ = self.calendar_detail_repo.create_detail(&entity)?;
        let json = CalendarDetailMapper::to_json(entity);
        Ok(json)
    }

    pub fn update_calendar_detail(
        &self,
        id: String,
        name: String,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut entity = match self.calendar_detail_repo.get_detail(&id) {
            Ok(Some(schema)) => schema.to_entity(),
            Ok(None) => return Err("Calendar not found".into()),
            Err(e) => return Err(Box::new(e)),
        };
        entity.name = name;
        let _ = self.calendar_detail_repo.update_detail(&entity)?;
        let json = CalendarDetailMapper::to_json(entity);
        Ok(json)
    }

    pub fn delete_calendar(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        match self.calendar_detail_repo.delete(id) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn export(&self, exporter: IcsExporter) -> Result<String, Box<dyn std::error::Error>> {
        let tmp_dir = "tmp";
        if !Path::new(tmp_dir).exists() {
            fs::create_dir(tmp_dir)?;
        }

        let calendars = self.calendar_detail_repo.kept_all()?;
        let entities = calendars.iter().map(|calendar| calendar.to_entity());
        let mut paths = Vec::new();
        entities.for_each(|entity: CalendarDetail| {
            let exporter_clone = exporter.clone();
            let path = format!("{}/{}.ics", tmp_dir, entity.name);
            exporter_clone.export(entity, &path).unwrap();
            paths.push(path);
        });
        Ok(paths.join(","))
    }

    pub fn get_all_events(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let events = self.calendar_event_repo.all()?;
        let json = events
            .iter()
            .map(|event| CalendarEventMapper::to_json(event.to_entity()))
            .collect::<Vec<_>>();
        Ok(Value::Array(json))
    }

    pub fn get_event(&self, id: String) -> Result<Value, Box<dyn std::error::Error>> {
        match self.calendar_event_repo.get_event(&id) {
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
        let calendar = self
            .calendar_detail_repo
            .get_detail_with_events(calendar_id.clone());
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
        let events = self.calendar_event_repo.recent(limit)?;
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
        let _ = self.calendar_event_repo.create_event(&entity)?;
        let json = CalendarEventMapper::to_json(entity);
        Ok(json)
    }

    pub fn update_event(
        &self,
        id: String,
        params: CalendarEventParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut entity = match self.calendar_event_repo.get_event(&id) {
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
        let _ = self.calendar_event_repo.update_event(&entity)?;
        let json = CalendarEventMapper::to_json(entity);
        Ok(json)
    }

    pub fn delete_event(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        match self.calendar_event_repo.delete_event(&id) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
