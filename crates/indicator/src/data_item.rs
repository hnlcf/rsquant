use quant_model::kline::Kline;
use ta::DataItem;

pub trait IntoDataItem {
    fn into_data_item(value: Self) -> Result<DataItem, Box<dyn std::error::Error>>;
}

impl IntoDataItem for Kline {
    fn into_data_item(value: Self) -> Result<DataItem, Box<dyn std::error::Error>> {
        let open = value.open_price.parse()?;
        let high = value.high_price.parse()?;
        let low = value.low_price.parse()?;
        let close = value.close_price.parse()?;
        let volume = value.volume.parse()?;

        Ok(DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()?)
    }
}
