const COMMANDS: &[&str] = &[
    "get_extension_metadata",
    "load_extension",
    "enable_torrent_support",
    "disable_torrent_support",
    "get_filters",
    "search",
    "get_series_info",
    "get_series_episodes",
    "get_series_videos",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
