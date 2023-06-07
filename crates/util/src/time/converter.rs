use chrono::TimeZone;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};

use super::{LocalTimeTool, UtcTimeTool, DATE_FORMAT_STR};

pub trait TimeConverter<Tz: TimeZone>
where
    Tz::Offset: std::fmt::Display,
{
    fn to_date_time(unix_time: i64) -> Option<DateTime<Tz>>;

    fn convert_to_date_time(unix_time: i64) -> Option<String> {
        Self::to_date_time(unix_time).map(|t| t.format(DATE_FORMAT_STR).to_string())
    }

    fn convert_to_unix_time(date_time: &str) -> Option<i64> {
        let date_time = NaiveDateTime::parse_from_str(date_time, DATE_FORMAT_STR).ok()?;
        Some(date_time.timestamp_millis())
    }
}

impl TimeConverter<Local> for LocalTimeTool {
    fn to_date_time(unix_time: i64) -> Option<DateTime<Local>> {
        let naive = NaiveDateTime::from_timestamp_millis(unix_time)?;
        let timezone_east = FixedOffset::east_opt(8 * 60 * 60)?;
        Some(DateTime::from_local(naive, timezone_east))
    }
}

impl TimeConverter<Utc> for UtcTimeTool {
    fn to_date_time(unix_time: i64) -> Option<DateTime<Utc>> {
        let naive = NaiveDateTime::from_timestamp_millis(unix_time)?;
        Some(DateTime::from_utc(naive, Utc))
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalTimeTool, TimeConverter};

    #[test]
    fn test_time_converter() {
        let expect_date_time = "2023-05-20 15:00:00";
        let expect_unix_time = LocalTimeTool::convert_to_unix_time(expect_date_time).unwrap_or(0);
        let actual_date_time =
            LocalTimeTool::convert_to_date_time(expect_unix_time).unwrap_or("".into());
        assert_eq!(actual_date_time, expect_date_time);
    }
}
