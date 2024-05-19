use std::collections::HashMap;

use binan_spot::http::request::Request as BinanRequest;
use quant_core::{
    Error,
    Result,
};

use crate::req::HttpClient;

pub trait AsyncSendReq: Clone {
    fn send_req(&self, client: &HttpClient) -> impl std::future::Future<Output = Result<String>>;
}

pub trait AsyncToString {
    fn async_to_string(self) -> impl std::future::Future<Output = Result<String>>;
}

impl AsyncSendReq for BinanRequest {
    async fn send_req(&self, client: &HttpClient) -> Result<String> {
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

pub struct Response {
    inner: reqwest::Response,
}

impl Response {
    pub fn new(inner: reqwest::Response) -> Self {
        Self { inner }
    }
}

impl AsyncToString for Response {
    async fn async_to_string(self) -> Result<String> {
        let headers: HashMap<String, String> =
            self.inner
                .headers()
                .iter()
                .fold(HashMap::new(), |mut headers, (k, v)| {
                    headers.entry(k.as_str().to_owned()).or_insert_with(|| {
                        v.to_str()
                            .expect("Failed to convert response header value to string")
                            .to_owned()
                    });
                    headers
                });
        let status = self.inner.status();
        let content = self
            .inner
            .text()
            .await
            .expect("Failed to get response content");

        if status.is_client_error() {
            let client_error =
                match serde_json::from_str::<binan_spot::http::error::BinanceApiError>(&content) {
                    Ok(err) => binan_spot::http::error::ClientError::Structured(
                        binan_spot::http::error::HttpError::new(status.as_u16(), err, headers),
                    ),
                    Err(_) => binan_spot::http::error::ClientError::Raw(
                        binan_spot::http::error::HttpError::new(status.as_u16(), content, headers),
                    ),
                };

            Err(Error::from(binan_spot::hyper::Error::Client(client_error)))
        } else if status.is_server_error() {
            Err(Error::from(binan_spot::hyper::Error::Server(
                binan_spot::http::error::HttpError::new(status.as_u16(), content, headers),
            )))
        } else {
            Ok(content)
        }
    }
}
