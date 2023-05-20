use chrono::{DateTime, Local, TimeZone, Utc};

use super::{LocalTimeTool, UtcTimeTool};

pub trait CurrentTime<Tz: TimeZone>
where
    Tz::Offset: std::fmt::Display,
{
    fn get_current() -> DateTime<Tz>;

    fn get_unix_time() -> u64 {
        Self::get_current().timestamp_millis() as u64
    }

    fn get_date_time() -> String {
        Self::get_current().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

impl CurrentTime<Local> for LocalTimeTool {
    fn get_current() -> DateTime<Local> {
        chrono::offset::Local::now()
    }
}

impl CurrentTime<Utc> for UtcTimeTool {
    fn get_current() -> DateTime<Utc> {
        chrono::offset::Utc::now()
    }
}
