use ta::{
    indicators::ExponentialMovingAverage as Ema,
    Close,
    DataItem,
    Next,
};

#[derive(Default)]
pub struct EmaOutputBuilder {
    output: Vec<f64>,
}

impl EmaOutputBuilder {
    pub fn compute(data: &[DataItem], period: usize) -> Self {
        let mut ema = Ema::new(period).unwrap();
        let f = |v: &DataItem| ema.next(v.close());
        Self {
            output: data.iter().map(f).collect(),
        }
    }

    pub fn build(self) -> Vec<f64> {
        self.output
    }
}
