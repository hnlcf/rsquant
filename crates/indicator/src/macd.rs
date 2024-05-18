use ta::{
    indicators::{
        MovingAverageConvergenceDivergence as Macd,
        MovingAverageConvergenceDivergenceOutput as MacdOutput,
    },
    Close,
    DataItem,
    Next,
};

#[derive(Default)]
pub struct MacdOutputBuilder {
    output: Vec<MacdOutput>,
}

impl MacdOutputBuilder {
    pub fn compute(data: &[DataItem]) -> Self {
        let mut macd = Macd::new(12, 26, 9).unwrap();
        let f = |v: &DataItem| macd.next(v.close());
        Self {
            output: data.iter().map(f).collect(),
        }
    }

    pub fn build(self) -> Vec<MacdOutput> {
        self.output
    }
}
