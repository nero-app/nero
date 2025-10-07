#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod types;

use std::{net::SocketAddr, sync::Arc};

use commands::*;
use nero_extensions::WasmExtension;
use nero_processors::{server::HttpServer, WasmProcessor};
use nero_wasm_host::{manager::ExtensionManager, WasmComponent};
use tauri::{Manager, Result};

const BASE_DIR: &str = "Nero";
const EXTENSIONS_DIR: &str = "Extensions";
const PROCESSORS_DIR: &str = "Processors";

pub struct AppState {
    pub extension: WasmExtension,
    pub processor: Arc<HttpServer>,
}

fn load_first_extension<T: WasmComponent>(app: &tauri::App, subdir: &str) -> Result<T> {
    let dir = app
        .path()
        .document_dir()
        .unwrap()
        .join(BASE_DIR)
        .join(subdir);

    let manager = ExtensionManager::new(dir)?;
    let extension = tauri::async_runtime::block_on(async {
        let extensions = manager.get_available_extensions().await?;
        manager.load_extension_async(&extensions[0].0).await
    })?;

    Ok(extension)
}

fn main() {
    tracing_subscriber::fmt().init();

    tauri::Builder::default()
        .setup(|app| {
            // For the moment, load the first extension found in the extensions directory.
            // TODO: if there are no extensions, open a screen with relevant information for
            // "how to load an extension".
            let extension = load_first_extension::<WasmExtension>(app, EXTENSIONS_DIR)?;
            let processor = load_first_extension::<WasmProcessor>(app, PROCESSORS_DIR)?;

            let addr = SocketAddr::from(([127, 0, 0, 1], 4321));
            let server = HttpServer::new(addr, processor);
            tauri::async_runtime::spawn({
                let server = server.clone();
                async move { server.run().await }
            });

            app.manage(AppState {
                extension,
                processor: server,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_filters,
            search,
            get_series_info,
            get_series_episodes,
            get_series_videos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
