use quant_model::kline::Kline;
use ta::DataItem;

pub trait IntoDataItem {
    fn into_data_item(&self) -> Result<DataItem, Box<dyn std::error::Error>>;
}

impl IntoDataItem for Kline {
    fn into_data_item(&self) -> Result<DataItem, Box<dyn std::error::Error>> {
        let open = self.open_price.parse()?;
        let high = self.high_price.parse()?;
        let low = self.low_price.parse()?;
        let close = self.close_price.parse()?;
        let volume = self.volume.parse()?;

        Ok(DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()?)
    }
}
