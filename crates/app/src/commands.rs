use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::State;

pub struct PlayerProcess(pub Mutex<Option<Child>>);

#[tauri::command]
#[tracing::instrument(skip(state))]
pub fn open_video_player(
    state: State<PlayerProcess>,
    player_path: String,
    url: String,
) -> Result<(), String> {
    if player_path.is_empty() {
        return Err("Player path cannot be empty".to_string());
    }

    let mut guard = state.0.lock().map_err(|_| "Mutex poisoned")?;

    if let Some(mut child) = guard.take() {
        let _ = child.kill();
    }

    #[cfg(target_os = "macos")]
    let exec_path = if player_path.ends_with(".app") {
        let app_name = player_path
            .trim_end_matches(".app")
            .split('/')
            .next_back()
            .unwrap_or("");
        format!("{}/Contents/MacOS/{}", player_path, app_name)
    } else {
        player_path
    };

    #[cfg(not(target_os = "macos"))]
    let exec_path = player_path;

    let child = Command::new(exec_path)
        .arg(&url)
        .spawn()
        .map_err(|e| format!("Failed to start player: {}", e))?;

    *guard = Some(child);

    Ok(())
}
