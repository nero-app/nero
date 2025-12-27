mod types;
mod utils;

use std::{net::SocketAddr, sync::Arc};

use nero_extensions::{WasmExtension, host::WasmHost};
use nero_processor::Processor;
use tauri::{
    Manager, Result, Runtime, State,
    plugin::{self, TauriPlugin},
};
use tokio::sync::RwLock;
use wasm_metadata::{Metadata, Payload};

use crate::{
    types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, Video},
    utils::AyncTryIntoWithState,
};

struct PluginState {
    host: WasmHost,
    extension: RwLock<Option<WasmExtension>>,
    processor: Arc<Processor>,
}

#[tauri::command]
#[tracing::instrument]
async fn get_extension_metadata(file_path: String) -> Result<Metadata> {
    let bytes = tokio::fs::read(file_path).await?;
    let payload = Payload::from_binary(&bytes)?;
    match payload {
        Payload::Component { metadata, .. } => Ok(metadata),
        Payload::Module(_) => Err(anyhow::anyhow!("unsupported wasm module").into()),
    }
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn load_extension(state: State<'_, PluginState>, file_path: String) -> Result<()> {
    let extension = state.host.load_extension_async(file_path).await?;
    state.extension.write().await.replace(extension);

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn get_filters(state: State<'_, PluginState>) -> Result<Vec<FilterCategory>> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("extension not loaded"))?;

    let categories = extension.filters().await?;
    Ok(categories.into_iter().map(Into::into).collect())
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn search(
    state: State<'_, PluginState>,
    query: &str,
    page: Option<u16>,
    filters: Vec<SearchFilter>,
) -> Result<SeriesPage> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("extension not loaded"))?;

    let ext_filters = filters.into_iter().map(Into::into).collect();
    let page = extension.search(query, page, ext_filters).await?;
    Ok(page.async_try_into_with_state(&state).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn get_series_info(state: State<'_, PluginState>, series_id: &str) -> Result<Series> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("extension not loaded"))?;

    let series = extension.get_series_info(series_id).await?;
    Ok(series.async_try_into_with_state(&state).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn get_series_episodes(
    state: State<'_, PluginState>,
    series_id: &str,
    page: Option<u16>,
) -> Result<EpisodesPage> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("extension not loaded"))?;

    let page = extension.get_series_episodes(series_id, page).await?;
    Ok(page.async_try_into_with_state(&state).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn get_series_videos(
    state: State<'_, PluginState>,
    series_id: &str,
    episode_id: &str,
) -> Result<Vec<Video>> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("extension not loaded"))?;

    let extension_videos = extension.get_series_videos(series_id, episode_id).await?;

    let mut videos = Vec::with_capacity(extension_videos.len());
    for video in extension_videos {
        videos.push(video.async_try_into_with_state(&state).await?);
    }
    Ok(videos)
}

pub struct Builder {
    processor_addr: SocketAddr,
}

impl Builder {
    pub fn new(processor_addr: SocketAddr) -> Self {
        Self { processor_addr }
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        let state = PluginState {
            host: WasmHost::default(),
            extension: RwLock::new(None),
            processor: Arc::new(Processor::new(self.processor_addr)),
        };

        plugin::Builder::new("nero-extensions")
            .setup(|app, _| {
                let processor = state.processor.clone();
                tauri::async_runtime::spawn(async move {
                    processor
                        .run()
                        .await
                        .expect("Unable to spawn internal extension processor server")
                });
                app.manage(state);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                get_extension_metadata,
                load_extension,
                get_filters,
                search,
                get_series_info,
                get_series_episodes,
                get_series_videos
            ])
            .build()
    }
}
