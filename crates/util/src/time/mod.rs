extern crate chrono;

mod converter;
mod current;
mod timezone;

pub use converter::TimeConverter;
pub use current::CurrentTime;
pub use timezone::TimeZoneConverter;

pub struct LocalTimeTool;
pub struct UtcTimeTool;

pub fn u64_to_datetime<'de, D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp: u64 = serde::Deserialize::deserialize(deserializer)?;
    let datetime = UtcTimeTool::to_date_time(timestamp as i64).unwrap_or_default();

    Ok(datetime)
}
