use binan_spot::http::request::Request as BinanRequest;

pub mod api_impl;
pub mod send_req;

pub use self::{
    api_impl::ApiImpl,
    send_req::Response,
};
use crate::{
    api::basic,
    Error,
};

#[derive(Debug)]
pub struct HttpClient {
    base_url: String,
    credentials: basic::Credentials,
    client: reqwest::Client,
}

impl HttpClient {
    pub async fn new(credentials: basic::Credentials) -> Self {
        let base_url = "https://api.binance.com";

        Self {
            base_url: base_url.into(),
            credentials,
            client: reqwest::Client::new(),
        }
    }

    pub async fn send<R: Into<BinanRequest>>(&self, request: R) -> Result<Response, Error> {
        let BinanRequest {
            method,
            path,
            params: origin_params,
            sign,
            ..
        } = request.into();

        let method = method.into();

        let mut header: Vec<(String, String)> = vec![(
            "Content-Type".into(),
            "application/json;charset=utf-8".into(),
        )];

        let mut params = origin_params;

        if sign {
            let origin_query = basic::create_query_string(&params);
            let signature = basic::sign(&origin_query, &self.credentials.signature)?;
            params.push(("signature".into(), signature));

            header.push(("X-MBX-APIKEY".into(), self.credentials.api_key.clone()));
        }

        let uri = url::Url::parse(&self.base_url)?.join(&path)?;
        let mut req_builder = self.client.request(method, uri).query(&params);
        for (k, v) in header {
            req_builder = req_builder.header(k, v);
        }
        let req = req_builder.build().unwrap();
        tracing::debug!("Client request: {:?}", req);

        self.client
            .execute(req)
            .await
            .map(Response::new)
            .map_err(|e| Error::Custom(e.to_string()))
    }
}
