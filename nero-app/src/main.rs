#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::*;
use nero_extensions::{ExtensionManager, WasmExtension};
use tauri::{async_runtime::block_on, Manager};

const EXTENSIONS_DIR: &str = "Nero";

pub struct AppState {
    pub extension: WasmExtension,
}

fn main() {
    tracing_subscriber::fmt().init();

    tauri::Builder::default()
        .setup(|app| {
            let extensions_dir = app.path().document_dir().unwrap().join(EXTENSIONS_DIR);
            let manager = ExtensionManager::new(extensions_dir)?;
            // For the moment, load the first extension found in the extensions directory.
            // TODO: if there are no extensions, open a screen with relevant information for
            // "how to load an extension".
            let first_extension = block_on(async {
                let extensions = manager.get_available_extensions().await?;
                manager.load_extension_async(&extensions[0].0).await
            });
            app.manage(AppState {
                extension: first_extension?,
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        // TODO: use portpicker to find an unused port
        .plugin(tauri_plugin_http_resources::init(
            "localhost".to_owned(),
            8080,
        ))
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
