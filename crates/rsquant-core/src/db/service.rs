use actix::{
    Actor,
    ActorFutureExt,
    Handler,
    ResponseActFuture,
    WrapFuture,
};
use sea_orm::{
    ActiveModelTrait,
    Database,
    DatabaseConnection,
    IntoActiveModel,
};

use crate::{
    entity::order,
    message::RecordOrderRequest,
    util::config,
    Error,
    Result,
};

#[derive(Default)]
pub struct DBService {
    conn: DatabaseConnection,
}

impl DBService {
    pub async fn from_config(config: config::DatabaseConfig) -> Result<Self> {
        match config.db_url.as_ref() {
            Some(url) => Database::connect(url)
                .await
                .map(|conn| Self { conn })
                .map_err(Error::from),
            e => Err(Error::Custom(format!("Invalid database url: {:?}", e))),
        }
    }
}

impl Actor for DBService {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // let conn = self.conn.clone();
        // actix::spawn(async move {
        //     init_db(&conn).await;
        // });

        tracing::info!("[db]: db actor started");
    }
}

async fn insert_order_record(
    conn: &DatabaseConnection,
    data: order::Model,
) -> Result<order::ActiveModel> {
    data.into_active_model()
        .save(conn)
        .await
        .map_err(Error::from)
}

impl Handler<RecordOrderRequest> for DBService {
    type Result = ResponseActFuture<Self, Result<order::ActiveModel>>;

    fn handle(&mut self, msg: RecordOrderRequest, _ctx: &mut Self::Context) -> Self::Result {
        let conn = self.conn.clone();
        async move { insert_order_record(&conn, msg.model).await }
            .into_actor(self)
            .boxed_local()
    }
}
