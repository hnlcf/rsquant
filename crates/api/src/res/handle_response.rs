use crate::res::BinanHttpClient;

use binan_spot::{http::request::Request, hyper::Response};

use std::process;

pub trait AsyncGetResp: Clone {
    fn get_response(
        &self,
        client: &BinanHttpClient,
    ) -> impl std::future::Future<Output = String> + Send;
}

pub trait AsyncToString {
    fn async_to_string(self) -> impl std::future::Future<Output = String> + Send;
}

impl AsyncGetResp for Request {
    async fn get_response(&self, client: &BinanHttpClient) -> String {
        loop {
            match client.send(self.to_owned()).await {
                Ok(res) => {
                    tracing::debug!("Send request from client.");
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
    async fn async_to_string(self) -> String {
        match self.into_body_str().await {
            Ok(s) => {
                tracing::debug!("Convert response into body string.");
                s
            }
            Err(e) => {
                tracing::error!("Failed to convert response body into string: {}.", e);
                process::abort();
            }
        }
    }
}
