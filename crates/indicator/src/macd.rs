use ta::{
    indicators::{
        MovingAverageConvergenceDivergence as Macd, MovingAverageConvergenceDivergenceOutput,
    },
    DataItem, Next, Open,
};

pub struct MacdOutput {
    output: Vec<MovingAverageConvergenceDivergenceOutput>,
}

impl MacdOutput {
    pub fn compute(data: Vec<DataItem>) -> Self {
        let mut macd = Macd::default();
        let f = |v: DataItem| macd.next(v.open());
        Self {
            output: data.into_iter().map(f).collect(),
        }
    }

    pub fn output(&self) -> Vec<MovingAverageConvergenceDivergenceOutput> {
        self.output.to_owned()
    }
}
