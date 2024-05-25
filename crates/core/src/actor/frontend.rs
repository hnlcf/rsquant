use std::sync::Arc;

use actix::{
    Actor,
    ActorContext,
    Addr,
    AsyncContext,
    Handler,
    StreamHandler,
};
use actix_web::{
    web,
    App,
    Error,
    HttpRequest,
    HttpResponse,
    HttpServer,
};
use actix_web_actors::ws;
use tokio::{
    sync::Mutex,
    time,
};

use crate::{
    message::{
        MultipleTickerApiRequest,
        MultipleTickerApiResponse,
        SubscribeTickerRequest,
    },
    QuantState,
};

#[derive(Debug, Default)]
pub struct SubscribeTickerActor {
    subscribe_req: Arc<Mutex<MultipleTickerApiRequest>>,
    exit_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl SubscribeTickerActor {
    fn update_req(&self, req: MultipleTickerApiRequest) {
        let ref_s = self.subscribe_req.clone();
        tokio::spawn(async move {
            let mut guard = ref_s.lock().await;
            *guard = req;
        });
    }

    fn start_impl(&mut self, self_addr: Addr<SubscribeTickerActor>) {
        let (exit_tx, mut exit_rx) = tokio::sync::oneshot::channel();
        self.exit_tx = Some(exit_tx);

        let req = self.subscribe_req.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut exit_rx => break,
                    tickers = QuantState::get_addr().send(req.lock().await.clone()) => {
                        if let Ok(Ok(tickers)) = tickers {
                            self_addr.send(tickers).await.unwrap();
                        }else {
                            tracing::error!("Failed to get tickers: {:?}", tickers);
                        }
                        time::sleep(time::Duration::from_secs(req.lock().await.interval)).await;
                    }
                }
            }

            tracing::debug!("Exit subscribe ticker loop");
        });
    }

    fn stop_impl(&mut self) {
        if let Some(tx) = self.exit_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl Actor for SubscribeTickerActor {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
///
/// When start to handle a new connection,
/// 1. Start an actor and return corresponding address
/// 3. Recieve a message that subscribe a ticker of symbol by given interval
/// 4. start a new tokio task to run:
///     1. get address by ctx
///     2. loop: get data, send data to address
/// 5. Actor handle data and send to frontend
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SubscribeTickerActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Text(text)) => {
                let subscribe_req: SubscribeTickerRequest = serde_json::from_str(&text).unwrap();
                self.update_req(subscribe_req);

                if self.exit_tx.is_none() {
                    self.start_impl(ctx.address());
                }
            }
            Ok(ws::Message::Close(_close)) => {
                self.stop_impl();
                ctx.stop();
            }
            _ => {
                tracing::warn!("Unknown message: {:?}", msg)
            }
        }
    }
}

impl Handler<MultipleTickerApiResponse> for SubscribeTickerActor {
    type Result = ();

    fn handle(&mut self, msg: MultipleTickerApiResponse, ctx: &mut Self::Context) {
        let data = serde_json::to_string(&msg.tickers).unwrap();
        ctx.text(data);
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (_, resp) = ws::WsResponseBuilder::new(SubscribeTickerActor::default(), &req, stream)
        .start_with_addr()?;
    Ok(resp)
}

pub async fn run_web() -> Result<(), Error> {
    let app = || App::new().route("/", web::get().to(index));
    tracing::info!("Start to run web server");

    HttpServer::new(app).bind("127.0.0.1:8000")?.run().await?;
    Ok(())
}
