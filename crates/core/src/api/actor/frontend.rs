use std::sync::Arc;

use actix::{
    Actor,
    ActorContext,
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
    api::message::{
        MultipleTickerApiRequest,
        MultipleTickerApiResponse,
        SubscribeTickerRequest,
    },
    QuantState,
};

#[derive(Debug, Default)]
struct SubscribeTicker {
    subscribe_req: Arc<Mutex<MultipleTickerApiRequest>>,
    exit_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl SubscribeTicker {
    fn update_req(&self, req: MultipleTickerApiRequest) {
        let ref_s = self.subscribe_req.clone();
        tokio::spawn(async move {
            let mut guard = ref_s.lock().await;
            *guard = req;
        });
    }
}

impl Actor for SubscribeTicker {
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
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SubscribeTicker {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Text(text)) => {
                let subscribe_req: SubscribeTickerRequest = serde_json::from_str(&text).unwrap();
                self.update_req(subscribe_req);

                if self.exit_tx.is_none() {
                    let (exit_tx, exit_rx) = tokio::sync::oneshot::channel();
                    self.exit_tx = Some(exit_tx);

                    let req = self.subscribe_req.clone();
                    let addr = ctx.address();
                    tokio::spawn(async move {
                        let mut exit_rx = exit_rx;
                        let addr = addr;

                        loop {
                            tokio::select! {
                                _ = &mut exit_rx => break,
                                tickers = QuantState::get().get_multi_ticker(req.lock().await.clone()) => {
                                    if let Ok(tickers) = tickers {
                                        addr.send(MultipleTickerApiResponse { tickers }).await.unwrap();
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
            }
            Ok(ws::Message::Close(_close)) => {
                if let Some(tx) = self.exit_tx.take() {
                    let _ = tx.send(());
                }
                ctx.stop();
            }
            _ => {
                tracing::warn!("Unknown message: {:?}", msg)
            }
        }
    }
}

impl Handler<MultipleTickerApiResponse> for SubscribeTicker {
    type Result = ();

    fn handle(&mut self, msg: MultipleTickerApiResponse, ctx: &mut Self::Context) {
        let data = serde_json::to_string(&msg.tickers).unwrap();
        ctx.text(data);
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (_, resp) =
        ws::WsResponseBuilder::new(SubscribeTicker::default(), &req, stream).start_with_addr()?;
    Ok(resp)
}

pub async fn run_web() -> Result<(), Error> {
    let app = || App::new().route("/", web::get().to(index));
    tracing::info!("Start to run web server");

    HttpServer::new(app).bind("127.0.0.1:8000")?.run().await?;
    Ok(())
}
