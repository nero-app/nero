use nodejs_installer::NodeJs;
use semver::VersionReq;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use tauri::async_runtime::RwLock;
use tauri::{AppHandle, Manager, Result, Runtime, State};
use webtorrent_sidecar::WebTorrent;

use crate::types::{NodeJsConfig, WebTorrentConfig};

pub struct PlayerProcess(pub Mutex<Option<Child>>);

#[tauri::command]
#[tracing::instrument(skip(state))]
pub fn open_video_player(
    state: State<PlayerProcess>,
    player_path: String,
    url: String,
) -> Result<()> {
    if player_path.is_empty() {
        return Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "Player path cannot be empty"
        )));
    }

    let mut guard = state
        .0
        .lock()
        .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;

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
        .map_err(|e| anyhow::anyhow!("Failed to start player: {}", e))?;

    *guard = Some(child);

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_handle, state))]
pub async fn initialize_nodejs<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, Arc<RwLock<Option<NodeJs>>>>,
    config: NodeJsConfig,
) -> Result<()> {
    let version_req = VersionReq::parse(">=18.0.0").unwrap();

    let node = match config {
        NodeJsConfig::Custom { node, npm } => {
            let node = NodeJs::from_paths(node, npm).await?;
            let node_version = node.node_version();

            if !version_req.matches(node_version) {
                return Err(tauri::Error::Anyhow(anyhow::anyhow!(
                    "WebTorrent version {node_version} doesn't match requirement {version_req}"
                )));
            }

            node
        }
        NodeJsConfig::AutoDetect { auto_download } => {
            let download_dir = app_handle.path().download_dir()?;
            if let Ok(nodejs) = NodeJs::from_local(&download_dir, &version_req).await {
                nodejs
            } else if let Ok(nodejs) = NodeJs::from_system(&version_req).await {
                nodejs
            } else if auto_download {
                NodeJs::download(&download_dir, &version_req).await?
            } else {
                return Err(tauri::Error::Anyhow(anyhow::anyhow!(
                    "WebTorrent {version_req} not found and auto_download is disabled"
                )));
            }
        }
    };

    tracing::info!("Initialized Node.js {node:?}");

    *state.write().await = Some(node);
    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_handle, node_state, webtorrent_state))]
pub async fn initialize_webtorrent<R: Runtime>(
    app_handle: AppHandle<R>,
    node_state: State<'_, Arc<RwLock<Option<NodeJs>>>>,
    webtorrent_state: State<'_, Arc<RwLock<Option<WebTorrent>>>>,
    config: WebTorrentConfig,
) -> Result<()> {
    let node_guard = node_state.read().await;
    let node = match node_guard.as_ref() {
        Some(node) => node,
        None => {
            return Err(tauri::Error::Anyhow(anyhow::anyhow!(
                "WebTorrent requires Node.js to be initialized"
            )))
        }
    };

    let version_req = VersionReq::parse(">=5.0.0, <6.0.0").unwrap();

    let webtorrent = match config {
        WebTorrentConfig::Custom { webtorrent } => {
            let webtorrent = WebTorrent::from_paths(node.node().to_path_buf(), webtorrent).await?;
            let webtorrent_version = webtorrent.version();

            if !version_req.matches(webtorrent_version) {
                return Err(tauri::Error::Anyhow(anyhow::anyhow!(
                    "WebTorrent version {webtorrent_version} doesn't match requirement {version_req}"
                )));
            }

            webtorrent
        }
        WebTorrentConfig::AutoDetect { auto_download } => {
            let download_dir = app_handle.path().download_dir()?;
            if let Ok(webtorrent) = WebTorrent::from_local(node, &download_dir, &version_req).await
            {
                webtorrent
            } else if auto_download {
                WebTorrent::download(node, &download_dir, &version_req).await?
            } else {
                return Err(tauri::Error::Anyhow(anyhow::anyhow!(
                    "WebTorrent {version_req} not found and auto_download is disabled"
                )));
            }
        }
    };

    *webtorrent_state.write().await = Some(webtorrent);

    Ok(())
}
