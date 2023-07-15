use crate::res::BinanHttpClient;

use binan_spot::{http::request::Request, hyper::Response};

use std::process;

pub trait AsyncGetResp: Clone {
    async fn get_response(&self, client: &BinanHttpClient) -> String;
}

pub trait AsyncToString {
    async fn async_to_string(self) -> String;
}

impl AsyncGetResp for Request {
    async fn get_response(&self, client: &BinanHttpClient) -> String {
        loop {
            match client.send(self.to_owned()).await {
                Ok(res) => {
                    log::debug!("Send request from client.");
                    return res.async_to_string().await;
                }
                Err(e) => {
                    log::error!("Failed to send request: {}. Resend it.", e);
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
                log::debug!("Convert response into body string.");
                s
            }
            Err(e) => {
                log::error!("Failed to convert response body into string: {}.", e);
                process::abort();
            }
        }
    }
}
