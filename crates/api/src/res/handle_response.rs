use binan_spot::{
    http::request::Request,
    hyper::Response,
};
use quant_core::{
    Error,
    Result,
};

use crate::res::BinanHttpClient;

pub trait AsyncGetResp: Clone {
    fn get_response(
        &self,
        client: &BinanHttpClient,
    ) -> impl std::future::Future<Output = Result<String>> + Send;
}

pub trait AsyncToString {
    fn async_to_string(self) -> impl std::future::Future<Output = Result<String>> + Send;
}

impl AsyncGetResp for Request {
    async fn get_response(&self, client: &BinanHttpClient) -> Result<String> {
        tracing::debug!("Send request: {:?}", self);
        loop {
            match client.send(self.to_owned()).await {
                Ok(res) => {
                    return res.async_to_string().await;
                }
                Err(e) => {
                    tracing::error!("Failed to send request: {}. Resend it.", e);
                    continue;
                }
            }
        }
    }
}

impl AsyncToString for Response {
    async fn async_to_string(self) -> Result<String> {
        self.into_body_str().await.map_err(Error::from)
    }
}
