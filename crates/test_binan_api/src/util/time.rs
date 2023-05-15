extern crate chrono;

use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};

pub struct TimeTool;

impl TimeTool {
    pub fn convert_to_date_time(unix_time: u64) -> Option<String> {
        let naive = NaiveDateTime::from_timestamp_opt(unix_time as i64, 0)?;
        let timezone_east = FixedOffset::east_opt(8 * 60 * 60)?;
        let datetime: DateTime<Local> = DateTime::from_local(naive, timezone_east);
        Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn convert_to_unix_time(date_time: &str) -> Option<u64> {
        let date_time = NaiveDateTime::parse_from_str(date_time, "%Y-%m-%d %H:%M:%S").ok()?;
        Some(date_time.timestamp_millis() as u64)
    }

    fn get_current() -> DateTime<Local> {
        chrono::offset::Local::now()
    }

    pub fn get_unix_time() -> u64 {
        TimeTool::get_current().timestamp_millis() as u64
    }

    pub fn get_date_time() -> String {
        TimeTool::get_current()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::TimeTool;

    #[test]
    fn test_time_converter() {
        let expect_date_time = "1970-01-01 00:00:00";
        let expect_unix_time = TimeTool::convert_to_unix_time(expect_date_time).unwrap_or(0);
        let actual_date_time =
            TimeTool::convert_to_date_time(expect_unix_time).unwrap_or("".into());
        assert_eq!(actual_date_time, expect_date_time);
    }
}
