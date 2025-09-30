#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::net::SocketAddr;

use commands::*;
use nero_extensions::WasmExtension;
use nero_processors::WasmProcessor;
use nero_wasm_host::manager::ExtensionManager;
use tauri::{async_runtime::block_on, Manager};

const BASE_DIR: &str = "Nero";
const EXTENSIONS_DIR: &str = "Extensions";
const PROCESSORS_DIR: &str = "Processors";

pub struct AppState {
    pub extension: WasmExtension,
    pub processor: WasmProcessor,
}

fn main() {
    tracing_subscriber::fmt().init();

    tauri::Builder::default()
        .setup(|app| {
            let extensions_dir = app
                .path()
                .document_dir()
                .unwrap()
                .join(BASE_DIR)
                .join(EXTENSIONS_DIR);
            let manager = ExtensionManager::new(extensions_dir)?;
            // For the moment, load the first extension found in the extensions directory.
            // TODO: if there are no extensions, open a screen with relevant information for
            // "how to load an extension".
            let first_extension = block_on(async {
                let extensions = manager.get_available_extensions().await?;
                manager.load_extension_async(&extensions[0].0).await
            });

            let processors_dir = app
                .path()
                .document_dir()
                .unwrap()
                .join(BASE_DIR)
                .join(PROCESSORS_DIR);
            let manager = ExtensionManager::new(processors_dir)?;
            let first_processor = block_on(async {
                let processors = manager.get_available_extensions().await?;
                manager.load_extension_async(&processors[0].0).await
            });

            let addr = SocketAddr::from(([127, 0, 0, 1], 4321));
            let processor: nero_processors::WasmProcessor = first_processor.unwrap();
            let server = nero_processors::server::HttpServer::new(addr, &processor).unwrap();
            tauri::async_runtime::spawn(async move { server.run().await });

            app.manage(AppState {
                extension: first_extension?,
                processor,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            resolve_resource,
            get_filters,
            search,
            get_series_info,
            get_series_episodes,
            get_series_videos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
