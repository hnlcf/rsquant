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

pub trait TimeConverter<Tz: TimeZone>
where
    Tz::Offset: std::fmt::Display,
{
    fn to_date_time(unix_time: i64) -> Option<DateTime<Tz>>;

    fn convert_to_date_time(unix_time: i64) -> Option<String> {
        Self::to_date_time(unix_time).map(|t| t.format(DEFAULT_DATETIME_FORMAT_STR).to_string())
    }

    fn convert_to_unix_time(date_time: &str) -> Option<i64> {
        let date_time = DateTime::parse_from_str(date_time, DEFAULT_DATETIME_FORMAT_STR).ok()?;
        Some(date_time.timestamp_millis())
    }
}

impl TimeConverter<Local> for LocalTimeTool {
    fn to_date_time(unix_time: i64) -> Option<DateTime<Local>> {
        let dt_utc = DateTime::from_timestamp_millis(unix_time)?;
        let dt = dt_utc.with_timezone(&Local);
        Some(dt)
    }
}

impl TimeConverter<Utc> for UtcTimeTool {
    fn to_date_time(unix_time: i64) -> Option<DateTime<Utc>> {
        let dt = DateTime::from_timestamp_millis(unix_time)?;
        Some(dt)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LocalTimeTool,
        TimeConverter,
    };

    #[test]
    fn test_time_converter() {
        let expect_date_time = "2023-05-20 15:00:00";
        let expect_unix_time = LocalTimeTool::convert_to_unix_time(expect_date_time).unwrap_or(0);
        let actual_date_time =
            LocalTimeTool::convert_to_date_time(expect_unix_time).unwrap_or("".into());
        assert_eq!(actual_date_time, expect_date_time);
    }
}
