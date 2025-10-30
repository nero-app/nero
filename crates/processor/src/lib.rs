mod error;
mod mime_detector;
mod routes;

use std::{io, net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};
use bytes::Bytes;
use http::Request;
use moka::future::Cache;
use tokio::net::TcpListener;
use tracing::debug;
use url::Url;
use uuid::Uuid;

use crate::{
    error::Error,
    mime_detector::mime_type,
    routes::{handle_image_request, handle_other_request, handle_video_request},
};

struct ServerState {
    addr: SocketAddr,
    http_client: reqwest::Client,
    http_requests: Cache<Uuid, Request<Option<Bytes>>>,
}

pub struct HttpServer {
    state: Arc<ServerState>,
}

impl HttpServer {
    pub fn new(addr: SocketAddr) -> Self {
        let state = ServerState {
            addr,
            http_client: reqwest::Client::new(),
            http_requests: Cache::builder().build(),
        };

        Self {
            state: Arc::new(state),
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        let app = Router::new()
            .route("/image/{request_id}", get(handle_image_request))
            .route("/video/{request_id}", get(handle_video_request))
            .route("/other/{request_id}", get(handle_other_request))
            .with_state(self.state.clone());

        let listener = TcpListener::bind(self.state.addr).await?;
        debug!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await
    }

    pub async fn handle_request(&self, request: Request<Option<Bytes>>) -> Result<Url, Error> {
        let uuid = Uuid::new_v4();
        self.state.http_requests.insert(uuid, request.clone()).await;

        match request.uri().scheme().unwrap().as_str() {
            "http" | "https" => self.handle_http_request(&uuid, &request).await,
            _ => Err(Error::UnsupportedScheme),
        }
    }

    async fn handle_http_request(
        &self,
        request_id: &Uuid,
        request: &Request<Option<Bytes>>,
    ) -> Result<Url, Error> {
        let mut base = Url::parse(&format!("http://{}", self.state.addr)).unwrap();

        if request.method() != http::Method::GET && request.method() != http::Method::HEAD {
            base.set_path(&format!("/other/{request_id}"));
            return Ok(base);
        }

        let mime_type = mime_type(&self.state.http_client, request)
            .await?
            .ok_or(Error::UnsupportedMediaType)?;

        match mime_type.type_() {
            mime::IMAGE => base.set_path(&format!("/image/{request_id}")),
            mime::VIDEO => base.set_path(&format!("/video/{request_id}")),
            mime::APPLICATION => {
                // let mime_type = mime_type.to_string();
                // base.set_path(&format!("/application/{mime_type}/{request_id}"));
                todo!()
            }
            _ => return Err(Error::UnsupportedMediaType),
        }

        Ok(base)
    }
}
