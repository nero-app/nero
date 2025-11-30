use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::State;

// TODO:
const SAMPLE_PLAYER_PATH: &str = "";

pub struct PlayerProcess(pub Mutex<Option<Child>>);

#[tauri::command]
#[tracing::instrument(skip(state))]
pub fn open_video_player(state: State<PlayerProcess>, url: String) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|_| "Mutex poisoned")?;

    if let Some(mut child) = guard.take() {
        let _ = child.kill();
    }

    let child = Command::new(SAMPLE_PLAYER_PATH)
        .arg(&url)
        .spawn()
        .map_err(|e| format!("Failed to start player: {}", e))?;

    *guard = Some(child);

    Ok(())
}
