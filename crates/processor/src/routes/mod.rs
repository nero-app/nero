mod image;
mod other;
mod video;

pub use image::*;
pub use other::*;
pub use video::*;

use axum::{body::Body, response::Response};
use http::HeaderMap;
use reqwest::Client;
use tracing::warn;

use crate::error::Error;

pub struct ForwardRequest {
    client: Client,
    url: String,
    headers: HeaderMap,
    body: Option<bytes::Bytes>,
}

impl ForwardRequest {
    pub fn new(client: Client, url: String, headers: HeaderMap) -> Self {
        Self {
            client,
            url,
            headers,
            body: None,
        }
    }

    pub fn body(mut self, body: bytes::Bytes) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> Result<Response, Error> {
        let mut req = self.client.get(self.url.clone()).headers(self.headers);
        if let Some(body) = self.body {
            req = req.body(body);
        }

        let res = req.send().await?;
        let status = res.status();
        if !status.is_success() {
            warn!("Remote server returned status {} for {}", status, self.url);
            return Err(Error::RemoteServer(status));
        }

        let headers = res.headers().clone();
        let stream = res.bytes_stream();
        let body = Body::from_stream(stream);

        let mut response = Response::new(body);
        *response.status_mut() = status;
        *response.headers_mut() = headers;
        Ok(response)
    }
}
