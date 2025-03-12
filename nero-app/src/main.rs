#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
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
