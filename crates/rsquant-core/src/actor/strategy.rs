use actix::{
    Actor,
    Handler,
};

use crate::{
    api::basic::TradeSide,
    message::KlineStrategyRequest,
    trade::Strategy,
    Result,
};

pub struct StrategyActor {
    inner: Box<dyn Strategy>,
}

impl StrategyActor {
    pub fn new(inner: Box<dyn Strategy>) -> Self {
        Self { inner }
    }
}

impl Actor for StrategyActor {
    type Context = actix::Context<Self>;
}

impl Handler<KlineStrategyRequest> for StrategyActor {
    type Result = Result<TradeSide>;

    fn handle(&mut self, msg: KlineStrategyRequest, _ctx: &mut Self::Context) -> Self::Result {
        let res = self.inner.check(&msg.data);
        tracing::debug!("[strategy:{}]: {:?}", self.inner.get_name(), res);
        Ok(res)
    }
}
