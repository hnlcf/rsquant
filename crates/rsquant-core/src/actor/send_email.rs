use actix::{
    Actor,
    ActorContext,
    Context,
    Handler,
};

use crate::{
    message::{
        NormalRequest,
        NormalResponse,
        SendEmailRequest,
    },
    util::{
        config::EmailConfig,
        email::{
            EmailBuilder,
            EmailManager,
        },
    },
    Error,
};

pub struct EmailActor {
    inner: EmailManager,
}

impl EmailActor {
    pub fn from_config(config: EmailConfig) -> Self {
        let inner = EmailBuilder::from_config(config).build();
        Self { inner }
    }
}

impl Actor for EmailActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("[email]: email actor started");
    }
}

impl Handler<SendEmailRequest> for EmailActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: SendEmailRequest, _: &mut Self::Context) -> Self::Result {
        let SendEmailRequest { subject, content } = msg;
        self.inner.send(&subject, &content)
    }
}

impl Handler<NormalRequest> for EmailActor {
    type Result = Result<NormalResponse, Error>;

    fn handle(&mut self, msg: NormalRequest, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            NormalRequest::Stop => {
                ctx.stop();
                Ok(NormalResponse::Success)
            }
        }
    }
}
