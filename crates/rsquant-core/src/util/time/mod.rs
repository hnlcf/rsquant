extern crate chrono;

mod converter;
mod current;
mod timezone;

pub use converter::TimeConverter;
pub use current::{
    CurrentTime,
    DurationInterval,
    GetDuration,
};
pub use timezone::TimeZoneConverter;

pub struct LocalTimeTool;
pub struct UtcTimeTool;

pub fn u64_to_datetime<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp: i64 = serde::Deserialize::deserialize(deserializer)?;
    let naive = LocalTimeTool::to_date_time(timestamp)
        .unwrap_or_default()
        .naive_local();

    Ok(naive)
}
