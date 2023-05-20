extern crate chrono;

use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};

pub struct TimeTool;
impl TimeTool {
    pub fn convert_utc_to_local(utc_time: u64) -> u64 {
        utc_time + 8 * 60 * 60 * 1000
    }

    pub fn convert_local_to_utc(local_time: u64) -> u64 {
        local_time - 8 * 60 * 60 * 1000
    }
}

pub struct LocalTimeTool;

impl LocalTimeTool {
    pub fn convert_to_date_time(unix_time: u64) -> Option<String> {
        let naive = NaiveDateTime::from_timestamp_millis(unix_time as i64)?;
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
        LocalTimeTool::get_current().timestamp_millis() as u64
    }

    pub fn get_date_time() -> String {
        LocalTimeTool::get_current()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}

pub struct UtcTimeTool;

impl UtcTimeTool {
    pub fn convert_to_date_time(unix_time: u64) -> Option<String> {
        let naive = NaiveDateTime::from_timestamp_millis(unix_time as i64)?;

        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn convert_to_unix_time(date_time: &str) -> Option<u64> {
        let date_time = NaiveDateTime::parse_from_str(date_time, "%Y-%m-%d %H:%M:%S").ok()?;
        Some(date_time.timestamp_millis() as u64)
    }

    fn get_current() -> DateTime<Utc> {
        chrono::offset::Utc::now()
    }

    pub fn get_unix_time() -> u64 {
        UtcTimeTool::get_current().timestamp_millis() as u64
    }

    pub fn get_date_time() -> String {
        UtcTimeTool::get_current()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::LocalTimeTool;

    #[test]
    fn test_time_converter() {
        let expect_date_time = "2023-05-20 15:00:00";
        let expect_unix_time = LocalTimeTool::convert_to_unix_time(expect_date_time).unwrap_or(0);
        let actual_date_time =
            LocalTimeTool::convert_to_date_time(expect_unix_time).unwrap_or("".into());
        assert_eq!(actual_date_time, expect_date_time);
    }
}
