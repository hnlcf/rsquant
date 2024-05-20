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

use crate::{
    api::message::{
        TickerApiRequest,
        TickerApiResponse,
    },
    model::ticker_price::TickerPrice,
};

struct FrontendApi;

impl Actor for FrontendApi {
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
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for FrontendApi {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let req: TickerApiRequest = serde_json::from_str(&text).unwrap();
                let addr = ctx.address();

                tokio::spawn(async move {
                    loop {
                        // let ticker = STATE
                        //     .get()
                        //     .get_ticker(req)
                        //     .await
                        //     .expect("Failed to get ticker");
                        let ticker = TickerPrice {
                            symbol: "BTCUSDT".to_string(),
                            price: "100".into(),
                        };

                        addr.send(TickerApiResponse { ticker }).await.unwrap();
                    }
                });
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<TickerApiResponse> for FrontendApi {
    type Result = ();

    fn handle(&mut self, msg: TickerApiResponse, ctx: &mut Self::Context) {
        let data = serde_json::to_string(&msg.ticker).unwrap();
        ctx.text(data);
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (addr, resp) = ws::WsResponseBuilder::new(FrontendApi, &req, stream)
        .start_with_addr()
        .unwrap();
    tracing::debug!("{:?}", resp);
    Ok(resp)
}

pub async fn run_web() {
    let app = || App::new().route("/ws/", web::get().to(index));

    HttpServer::new(app)
        .bind(("127.0.0.1", 8080))
        .expect("Can not bind to 8080")
        .run()
        .await
        .expect("Failed to run server")
}
