use std::fs::File;
use std::io::Write;

use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike};

pub struct IcsExporter {}

impl IcsExporter {
    pub fn new() -> IcsExporter {
        IcsExporter {}
    }

    pub fn export(self, entity: CalendarDetail, file_path: &str) -> std::io::Result<()> {
        let calendar = self.to_ical(entity);
        let mut file = File::create(file_path)?;
        file.write_all(calendar.to_string().as_bytes())?;
        Ok(())
    }

    fn to_ical(&self, entity: CalendarDetail) -> Calendar {
        let mut calendar = Calendar::new();
        calendar.name(&entity.name);
        for event in &entity.events {
            calendar.push(Self::event_to_ical(event));
        }
        calendar
    }

    fn event_to_ical(event: &CalendarEvent) -> Event {
        Event::new()
            .summary(&event.summary)
            .description(&event.description)
            .starts(CalendarDateTime::from(event.start_at))
            .ends(CalendarDateTime::from(event.end_at))
            .class(Class::Public)
            .location(event.location.as_deref().unwrap_or(""))
            .done()
    }
}
