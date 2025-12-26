mod error;
mod mime_detector;
mod routes;
mod utils;

use std::{io, net::SocketAddr, sync::Arc};

use anyhow::bail;
use axum::{Router, routing::get};
use bytes::Bytes;
use http::{Request, uri::Scheme};
use moka::future::Cache;
use tokio::{net::TcpListener, sync::RwLock};
use tracing::debug;
use url::Url;
use webtorrent_sidecar::WebTorrent;

use crate::{
    mime_detector::mime_type,
    routes::{handle_image_request, handle_video_request},
    utils::get_request_hash,
};

#[derive(Clone)]
enum VideoRequestSource {
    Http(Box<Request<Option<Bytes>>>),
    MagnetUri(String),
}

struct ServerState {
    addr: SocketAddr,
    http_client: reqwest::Client,
    image_requests: Cache<u64, Request<Option<Bytes>>>,
    video_requests: Cache<u64, VideoRequestSource>,
    torrent_handler: RwLock<Option<WebTorrent>>,
}

pub struct Processor {
    state: Arc<ServerState>,
}

impl Processor {
    pub fn new(addr: SocketAddr) -> Self {
        let state = ServerState {
            addr,
            http_client: reqwest::Client::new(),
            // TODO: ttls
            image_requests: Cache::builder().build(),
            video_requests: Cache::builder().build(),
            torrent_handler: RwLock::new(None),
        };

        Self {
            state: Arc::new(state),
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        let app = Router::new()
            .route("/image/{request_id}", get(handle_image_request))
            .route("/video/{request_id}", get(handle_video_request))
            .with_state(self.state.clone());

        let listener = TcpListener::bind(self.state.addr).await?;
        debug!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await
    }

    pub async fn enable_torrent_support(&self, webtorrent: WebTorrent) {
        *self.state.torrent_handler.write().await = Some(webtorrent);
    }

    pub async fn disable_torrent_support(&self) {
        *self.state.torrent_handler.write().await = None;
    }

    pub async fn register_image_request(
        &self,
        request: Request<Option<Bytes>>,
    ) -> anyhow::Result<Url> {
        let mime_type = mime_type(&self.state.http_client, &request)
            .await?
            .ok_or(anyhow::anyhow!("Could not detect mime type"))?;

        if mime_type.subtype() == "application/x-bittorrent" {
            bail!("Torrents are not supported for images");
        }

        let request_hash = get_request_hash(&request);
        let url = Url::parse(&format!(
            "{}://{}/image/{request_hash}",
            Scheme::HTTP,
            self.state.addr,
        ))?;

        self.state
            .image_requests
            .insert(request_hash, request)
            .await;

        Ok(url)
    }

    pub async fn register_video_request(
        &self,
        request: Request<Option<Bytes>>,
    ) -> anyhow::Result<Url> {
        let mime_type = mime_type(&self.state.http_client, &request)
            .await?
            .ok_or(anyhow::anyhow!("Could not detect mime type"))?;

        let request_hash = get_request_hash(&request);
        let mut base = Url::parse(&format!("{}://{}", Scheme::HTTP, self.state.addr))?;

        match mime_type.type_() {
            mime::VIDEO => base.set_path(&format!("/video/{request_hash}")),
            mime::APPLICATION if mime_type.subtype() == "application/x-bittorrent" => todo!(),
            _ => bail!("Unsupported media type"),
        }

        self.state
            .video_requests
            .insert(request_hash, VideoRequestSource::Http(Box::new(request)))
            .await;

        Ok(base)
    }

    pub async fn register_video_magnet(&self, uri: String) -> anyhow::Result<Url> {
        todo!()
    }
}
