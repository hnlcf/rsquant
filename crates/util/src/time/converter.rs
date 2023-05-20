use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};

use super::{LocalTimeTool, UtcTimeTool, DATE_FORMAT_STR};

pub trait TimeConverter {
    fn convert_to_date_time(unix_time: u64) -> Option<String>;

    fn convert_to_unix_time(date_time: &str) -> Option<u64> {
        let date_time = NaiveDateTime::parse_from_str(date_time, DATE_FORMAT_STR).ok()?;
        Some(date_time.timestamp_millis() as u64)
    }
}

impl TimeConverter for LocalTimeTool {
    fn convert_to_date_time(unix_time: u64) -> Option<String> {
        let naive = NaiveDateTime::from_timestamp_millis(unix_time as i64)?;
        let timezone_east = FixedOffset::east_opt(8 * 60 * 60)?;
        let datetime: DateTime<Local> = DateTime::from_local(naive, timezone_east);
        Some(datetime.format(DATE_FORMAT_STR).to_string())
    }
}
impl TimeConverter for UtcTimeTool {
    fn convert_to_date_time(unix_time: u64) -> Option<String> {
        let naive = NaiveDateTime::from_timestamp_millis(unix_time as i64)?;

        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        Some(datetime.format(DATE_FORMAT_STR).to_string())
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
