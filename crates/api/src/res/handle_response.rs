use crate::res::BinanHttpClient;

use binan_spot::{http::request::Request, hyper::Response};
use serde::Deserialize;

use std::process::abort;

pub struct HandleResponse;

impl HandleResponse {
    pub fn decode_response<'a, T: Deserialize<'a>>(data: &'a str) -> T {
        match serde_json::from_str(data) {
            Ok(t) => {
                log::info!("Deserialize response string to data structure.");
                t
            }
            Err(e) => {
                log::error!(
                    "Failed to deserialize response string to data structure: {}.",
                    e
                );
                abort();
            }
        }
    }

    pub async fn get_response(client: &BinanHttpClient, request: impl Into<Request>) -> String {
        let request = request.into();
        loop {
            match client.send(request.clone()).await {
                Ok(res) => {
                    log::info!("Send request from client.");
                    return Self::response_to_string(res).await;
                }
                Err(e) => {
                    log::error!("Failed to send request: {}. Resend it.", e);
                    continue;
                }
            }
        }
    }

    pub async fn response_to_string(res: Response) -> String {
        match res.into_body_str().await {
            Ok(s) => {
                log::info!("Convert response into body string.");
                s
            }
            Err(e) => {
                log::error!("Failed to convert response body into string: {}.", e);
                abort();
            }
        }
    }
}
