extern crate chrono;

mod converter;
mod current;
mod timezone;

pub use converter::TimeConverter;
pub use current::CurrentTime;
pub use timezone::TimeZoneConverter;

const DATE_FORMAT_STR: &str = "%Y-%m-%d %H:%M:%S";

pub struct LocalTimeTool;
pub struct UtcTimeTool;
