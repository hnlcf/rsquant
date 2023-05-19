use serde::Deserialize;

use binan_spot::http::request::Request;

use crate::res::BinanHttpClient;

pub struct HandleResponse;

impl HandleResponse {
    pub async fn decode_response<'a, T: Deserialize<'a>>(data: &'a str) -> T {
        serde_json::from_str(data).expect("Failed to parse response.")
    }

    pub async fn get_response(client: &BinanHttpClient, request: impl Into<Request>) -> String {
        client
            .send(request)
            .await
            .expect("Failed to send request.")
            .into_body_str()
            .await
            .expect("Failed to convert response body into string.")
    }
}
