extern crate chrono;

use chrono::prelude::*;

pub struct TimeConverter;

pub struct CurrentTime;

impl TimeConverter {
    pub fn unix_time_to_date(unix_time: u64) -> Option<String> {
        let naive = NaiveDateTime::from_timestamp_opt(unix_time as i64, 0)?;
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn date_to_unix_time(datetime: &str) -> Option<u64> {
        let date_time = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").ok()?;
        Some(date_time.timestamp_millis() as u64)
    }
}

impl CurrentTime {
    fn get_current() -> DateTime<Utc> {
        chrono::offset::Utc::now()
    }

    pub fn get_unix_time() -> u64 {
        CurrentTime::get_current().timestamp_millis() as u64
    }

    pub fn get_date_time() -> String {
        CurrentTime::get_current()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}
