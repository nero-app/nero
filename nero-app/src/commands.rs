use nero_extensions::types::{
    EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, Video,
};
use tauri::{Result, State};

use crate::AppState;

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_filters(state: State<'_, AppState>) -> Result<Vec<FilterCategory>> {
    Ok(state.extension.filters().await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn search(
    state: State<'_, AppState>,
    query: &str,
    page: Option<u16>,
    filters: Vec<SearchFilter>,
) -> Result<SeriesPage> {
    Ok(state.extension.search(query, page, filters).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_info(state: State<'_, AppState>, series_id: &str) -> Result<Series> {
    Ok(state.extension.get_series_info(series_id).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_episodes(
    state: State<'_, AppState>,
    series_id: &str,
    page: Option<u16>,
) -> Result<EpisodesPage> {
    Ok(state.extension.get_series_episodes(series_id, page).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_videos(
    state: State<'_, AppState>,
    series_id: &str,
    episode_id: &str,
) -> Result<Vec<Video>> {
    Ok(state
        .extension
        .get_series_videos(series_id, episode_id)
        .await?)
}
