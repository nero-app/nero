mod error;
mod mime_detector;
mod routes;

use std::{io, net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};
use bytes::Bytes;
use http::Request;
use magnet_uri::MagnetURI;
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

pub struct Processor {
    state: Arc<ServerState>,
}

impl Processor {
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

    pub async fn handle_http_request(&self, request: Request<Option<Bytes>>) -> Result<Url, Error> {
        let scheme = request.uri().scheme().map(|s| s.as_str());
        if scheme != Some("http") && scheme != Some("https") {
            return Err(Error::UnsupportedScheme);
        }

        if request.headers().is_empty() {
            return Ok(Url::parse(&request.uri().to_string())?);
        }

        let uuid = Uuid::new_v4();
        let mut base = Url::parse(&format!("http://{}", self.state.addr)).unwrap();

        if request.method() != http::Method::GET {
            base.set_path(&format!("/other/{uuid}"));
            return Ok(base);
        }

        let mime_type = mime_type(&self.state.http_client, &request)
            .await?
            .ok_or(Error::UnsupportedMediaType)?;

        match mime_type.type_() {
            mime::IMAGE => base.set_path(&format!("/image/{uuid}")),
            mime::VIDEO => base.set_path(&format!("/video/{uuid}")),
            mime::APPLICATION => {
                // let mime_type = mime_type.to_string();
                // base.set_path(&format!("/application/{mime_type}/{uuid}"));
                todo!()
            }
            _ => return Err(Error::UnsupportedMediaType),
        }

        self.state.http_requests.insert(uuid, request).await;

        Ok(base)
    }

    // TODO:
    #[allow(unused_variables)]
    pub fn handle_magnet_uri(uri: MagnetURI) -> Result<Url, Error> {
        todo!()
    }
}
