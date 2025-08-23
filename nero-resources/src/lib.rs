// TODO: Remove this once we're done with the new basic setup
#![allow(unused)]

mod proxy;
mod utils;

use axum::{Router, routing::get};
use reqwest::Client;
use tauri::{
    Runtime,
    plugin::{Builder as PluginBuilder, TauriPlugin},
};
use tokio::net::TcpListener;

use crate::proxy::init;

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
        PluginBuilder::new("tauri-plugin-http-resources")
            .setup(move |_app, _api| {
                tauri::async_runtime::spawn(init(self.host, self.port));
                Ok(())
            })
            .build()
    }
}
