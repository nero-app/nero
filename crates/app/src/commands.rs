use tauri::{Result, State};

use crate::{
    types::{
        AyncTryIntoWithState, EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, Video,
    },
    AppState,
};

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_filters(state: State<'_, AppState>) -> Result<Vec<FilterCategory>> {
    let categories = state.extension.filters().await?;
    Ok(categories.into_iter().map(Into::into).collect())
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn search(
    state: State<'_, AppState>,
    query: &str,
    page: Option<u16>,
    filters: Vec<SearchFilter>,
) -> Result<SeriesPage> {
    let ext_filters = filters.into_iter().map(Into::into).collect();
    let page = state.extension.search(query, page, ext_filters).await?;
    Ok(page.async_try_into_with_state(&state).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_info(state: State<'_, AppState>, series_id: &str) -> Result<Series> {
    let series = state.extension.get_series_info(series_id).await?;
    Ok(series.async_try_into_with_state(&state).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_episodes(
    state: State<'_, AppState>,
    series_id: &str,
    page: Option<u16>,
) -> Result<EpisodesPage> {
    let page = state.extension.get_series_episodes(series_id, page).await?;
    Ok(page.async_try_into_with_state(&state).await?)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_videos(
    state: State<'_, AppState>,
    series_id: &str,
    episode_id: &str,
) -> Result<Vec<Video>> {
    let extension_videos = state
        .extension
        .get_series_videos(series_id, episode_id)
        .await?;

    let mut videos = Vec::with_capacity(extension_videos.len());
    for video in extension_videos {
        videos.push(video.async_try_into_with_state(&state).await?);
    }
    Ok(videos)
}
