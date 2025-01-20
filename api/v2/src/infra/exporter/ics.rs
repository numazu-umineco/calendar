use std::fs::File;
use std::io::Write;

use crate::domain::entities::calendar_detail::CalendarDetail;
use crate::domain::entities::calendar_event::CalendarEvent;
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};

pub struct IcsExporter {
    detail: CalendarDetail,
}

impl IcsExporter {
    pub fn new(detail: CalendarDetail) -> IcsExporter {
        IcsExporter { detail: detail }
    }

    pub fn export(self, file_path: &str) -> std::io::Result<()> {
        let calendar = self.to_ical();
        let mut file = File::create(file_path)?;
        file.write_all(calendar.to_string().as_bytes())?;
        Ok(())
    }

    pub fn to_ical(&self) -> Calendar {
        let mut calendar = Calendar::new();
        calendar.name(&self.detail.name);
        for event in &self.detail.events {
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
            .location(&event.location)
            .done()
    }
}
