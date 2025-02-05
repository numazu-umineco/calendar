use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CalendarDetailParams {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CalendarEventParams {
    pub id: String,
    pub summary: String,
    pub description: String,
    pub location: Option<String>,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub all_day: bool,
    pub calendar_id: String,
    pub url: Option<String>,
}
