use test_binan_api::util::time::TimeTool;

pub struct DateTime;

impl DateTime {
    pub fn get_current() -> (String, u64) {
        let date_time = TimeTool::get_date_time();
        let unix_time = TimeTool::convert_to_unix_time(&date_time).unwrap_or(0);
        (date_time, unix_time)
    }
}
