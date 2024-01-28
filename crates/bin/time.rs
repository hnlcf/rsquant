use quant_util::time::{CurrentTime, LocalTimeTool, TimeConverter};

pub struct DateTime;

impl DateTime {
    pub fn get_local_current() -> (String, u64) {
        let date_time = LocalTimeTool::get_date_time();
        let unix_time = LocalTimeTool::convert_to_unix_time(&date_time).unwrap_or(0);
        (date_time, unix_time as u64)
    }
}
