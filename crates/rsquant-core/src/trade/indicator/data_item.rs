use ta::DataItem;

use crate::model::kline::Kline;

pub trait ToDataItem {
    fn to_data_item(&self) -> Result<DataItem, Box<dyn std::error::Error>>;
}

impl ToDataItem for Kline {
    fn to_data_item(&self) -> Result<DataItem, Box<dyn std::error::Error>> {
        let open: f64 = fast_float::parse(&self.open_price)?;
        let high: f64 = fast_float::parse(&self.high_price)?;
        let low: f64 = fast_float::parse(&self.low_price)?;
        let close: f64 = fast_float::parse(&self.close_price)?;
        let volume: f64 = fast_float::parse(&self.volume)?;

        Ok(DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()?)
    }
}
