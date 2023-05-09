extern crate chrono;

use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};

pub struct TimeConverter;

pub struct CurrentTime;

impl TimeConverter {
    pub fn unix_time_to_date(unix_time: u64) -> Option<String> {
        let naive = NaiveDateTime::from_timestamp_opt(unix_time as i64, 0)?;
        let timezone_east = FixedOffset::east_opt(8 * 60 * 60)?;
        let datetime: DateTime<Local> = DateTime::from_local(naive, timezone_east);
        Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn date_to_unix_time(datetime: &str) -> Option<u64> {
        let date_time = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").ok()?;
        Some(date_time.timestamp_millis() as u64)
    }
}

impl CurrentTime {
    fn get_current() -> DateTime<Local> {
        chrono::offset::Local::now()
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
