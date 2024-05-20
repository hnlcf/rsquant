use std::collections::HashMap;

use actix::{
    Actor,
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
use tokio::time;

use crate::{
    api::message::{
        SubscribeTickerRequest,
        TickerApiResponse,
    },
    QuantState,
};

#[derive(Debug, Default)]
struct SubscribeTicker {
    subscribe_table: HashMap<String, tokio::sync::oneshot::Sender<bool>>,
}

impl SubscribeTicker {
    fn subscribe(&mut self, symbol: String) -> tokio::sync::oneshot::Receiver<bool> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.subscribe_table.insert(symbol, tx);
        rx
    }

    fn unsubscribe(&mut self, symbol: &str) {
        let _ = self
            .subscribe_table
            .remove(symbol)
            .map(|tx| tx.send(true).expect("Failed to send unsubscribe signal"));
    }

    fn unsubscribe_all(&mut self) {
        for (_, tx) in self.subscribe_table.drain() {
            let _ = tx.send(true);
        }
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
                let req: SubscribeTickerRequest = serde_json::from_str(&text).unwrap();
                match req {
                    SubscribeTickerRequest::Subscribe(ticker_req) => {
                        let addr = ctx.address();
                        let req = ticker_req;
                        let exit_rx = self.subscribe(req.symbol.clone());

                        tokio::spawn(async move {
                            let mut exit_rx = exit_rx;

                            tracing::debug!("Subscribe ticker: {:?}", req);
                            loop {
                                tokio::select! {
                                    _ = &mut exit_rx => break,
                                    ticker = QuantState::get().get_ticker(req.clone()) => {
                                        addr.send(TickerApiResponse { ticker: ticker.unwrap() }).await.unwrap();
                                        time::sleep(time::Duration::from_secs(req.interval)).await;
                                    }
                                }
                            }
                            tracing::debug!("Unsubscribe ticker: {:?}", req);
                        });
                    }
                    SubscribeTickerRequest::Unsubscribe(symbol) => {
                        self.unsubscribe(&symbol);
                    }
                }
            }
            Ok(ws::Message::Close(_close)) => {
                self.unsubscribe_all();
            }
            _ => (),
        }
    }
}

impl Handler<TickerApiResponse> for SubscribeTicker {
    type Result = ();

    fn handle(&mut self, msg: TickerApiResponse, ctx: &mut Self::Context) {
        let data = serde_json::to_string(&msg.ticker).unwrap();
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
