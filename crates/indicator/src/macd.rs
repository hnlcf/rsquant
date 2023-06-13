use ta::{
    indicators::{
        MovingAverageConvergenceDivergence as Macd,
        MovingAverageConvergenceDivergenceOutput as MacdOutput,
    },
    Close, DataItem, Next,
};

#[derive(Default)]
pub struct MacdOutputBuilder {
    output: Vec<MacdOutput>,
}

impl MacdOutputBuilder {
    pub fn compute(self, data: &[DataItem]) -> Self {
        let mut macd = Macd::default();
        let f = |v: &DataItem| macd.next(v.close());
        Self {
            output: data.iter().map(f).collect(),
        }
    }

    pub fn build(self) -> Vec<MacdOutput> {
        self.output
    }
}
