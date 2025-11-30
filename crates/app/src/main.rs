#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::{net::SocketAddr, sync::Mutex};

use crate::commands::PlayerProcess;

fn main() {
    tracing_subscriber::fmt().init();

    let processor_port = portpicker::pick_unused_port()
        .expect("failed to pick an unused port for the extensions processor");
    let processor_addr = SocketAddr::from(([127, 0, 0, 1], processor_port));

    tauri::Builder::default()
        .manage(PlayerProcess(Mutex::new(None)))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_nero_extensions::Builder::new(processor_addr).build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![commands::open_video_player])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
