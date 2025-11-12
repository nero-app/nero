#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;

fn main() {
    tracing_subscriber::fmt().init();

    let processor_addr = SocketAddr::from(([127, 0, 0, 1], 4321));

    tauri::Builder::default()
        .plugin(tauri_plugin_nero_extensions::Builder::new(processor_addr).build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
