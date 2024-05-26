use ta::{
    indicators::RelativeStrengthIndex as Rsi,
    Close,
    DataItem,
    Next,
};

#[derive(Default)]
pub struct RsiOutputBuilder {
    output: Vec<f64>,
}

impl RsiOutputBuilder {
    pub fn compute(data: &[DataItem], period: usize) -> Self {
        let mut rsi = Rsi::new(period).unwrap();
        let f = |v: &DataItem| rsi.next(v.close());
        Self {
            output: data.iter().map(f).collect(),
        }
    }

    pub fn build(self) -> Vec<f64> {
        self.output
    }
}
