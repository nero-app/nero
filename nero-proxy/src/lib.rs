// TODO: Remove this once we're done with the new basic setup
#![allow(unused)]

mod request;
mod utils;

use std::sync::Arc;

use axum::{Router, routing::get};
use reqwest::Client;
use tauri::{
    Runtime,
    plugin::{Builder as PluginBuilder, TauriPlugin},
};
use tokio::net::TcpListener;

use crate::request::proxy_request;

#[derive(Clone)]
pub struct ProxyState {
    host: String,
    port: u16,
    http_client: Arc<Client>,
}

pub async fn init(host: String, port: u16) {
    let http_client = Client::new();
    let state = ProxyState {
        host: host.clone(),
        port,
        http_client: Arc::new(http_client),
    };

    let app = Router::new()
        .route("/", get(proxy_request))
        .with_state(state);

    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}

// TODO: Add tauri command to get the proxy URL from the frontend

pub struct Builder {
    host: String,
    port: u16,
}

impl Builder {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        PluginBuilder::new("tauri-plugin-video-proxy")
            .setup(move |_app, _api| {
                tauri::async_runtime::spawn(init(self.host, self.port));
                Ok(())
            })
            .build()
    }
}
