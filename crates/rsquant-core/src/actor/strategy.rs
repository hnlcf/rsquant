use actix::{
    Actor,
    Handler,
};

use crate::{
    entity::side,
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

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!(
            "[strategy:{}]: strategy actor started",
            self.inner.get_name()
        );
    }
}

impl Handler<KlineStrategyRequest> for StrategyActor {
    type Result = Result<side::TradeSide>;

    fn handle(&mut self, msg: KlineStrategyRequest, _ctx: &mut Self::Context) -> Self::Result {
        let res = self.inner.check(&msg.data);
        tracing::info!("[strategy:{}]: {:?}", self.inner.get_name(), res);
        Ok(res)
    }
}
