use icalendar::Calendar;
use std::fs::File;
use std::io::Write;

pub struct IcsExporter;

impl IcsExporter {
    pub fn new() -> IcsExporter {
        IcsExporter
    }

    pub fn execute(self, calendar: Calendar, file_path: &str) -> std::io::Result<()> {
        let mut file = File::create(file_path)?;
        file.write_all(calendar.to_string().as_bytes())?;
        Ok(())
    }
}
