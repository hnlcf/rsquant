use chrono::{
    DateTime,
    Local,
    TimeZone,
    Utc,
};

use crate::util::{
    constants::DEFAULT_DATETIME_FORMAT_STR,
    time::{
        LocalTimeTool,
        UtcTimeTool,
    },
};

pub trait CurrentTime<Tz: TimeZone>
where
    Tz::Offset: std::fmt::Display,
{
    fn get_current() -> DateTime<Tz>;

    fn get_unix_time() -> u64 {
        Self::get_current().timestamp_millis() as u64
    }

    fn get_date_time() -> String {
        Self::get_current()
            .format(DEFAULT_DATETIME_FORMAT_STR)
            .to_string()
    }
}

impl CurrentTime<Local> for LocalTimeTool {
    fn get_current() -> DateTime<Local> {
        Local::now()
    }
}

impl CurrentTime<Utc> for UtcTimeTool {
    fn get_current() -> DateTime<Utc> {
        Utc::now()
    }
}

pub enum DurationInterval {
    Seconds1,
    Minutes1,
    Hours1,
    Days1,
    Weeks1,
    Months1,
    Years1,
}

pub trait GetDuration {
    fn get_duration(&self, interval: DurationInterval, count: i64) -> (u64, u64);
}

impl GetDuration for UtcTimeTool {
    fn get_duration(&self, interval: DurationInterval, count: i64) -> (u64, u64) {
        let current = Self::get_current();
        let start = match interval {
            DurationInterval::Seconds1 => current - chrono::Duration::seconds(count),
            DurationInterval::Minutes1 => current - chrono::Duration::minutes(count),
            DurationInterval::Hours1 => current - chrono::Duration::hours(count),
            DurationInterval::Days1 => current - chrono::Duration::days(count),
            DurationInterval::Weeks1 => current - chrono::Duration::weeks(count),
            DurationInterval::Months1 => current - chrono::Duration::days(count * 30),
            DurationInterval::Years1 => current - chrono::Duration::days(count * 365),
        };
        (
            start.timestamp_millis() as u64,
            current.timestamp_millis() as u64,
        )
    }
}
