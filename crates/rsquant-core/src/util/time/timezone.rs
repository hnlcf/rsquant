pub struct TimeZoneConverter;

impl TimeZoneConverter {
    pub fn convert_utc_to_local(utc_time: u64) -> u64 {
        utc_time + 8 * 60 * 60 * 1000
    }

    pub fn convert_local_to_utc(local_time: u64) -> u64 {
        local_time - 8 * 60 * 60 * 1000
    }
}
