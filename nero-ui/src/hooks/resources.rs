use nero_extensions::{
    types::{FilterCategory, Series, SeriesVideo},
    Extension as ExtensionTrait,
};
use sycamore::web::{create_client_resource, Resource};
use wasm_bindgen::UnwrapThrowExt;

use crate::extensions::Extension;

// TODO: Handle errors

pub fn use_filters() -> Resource<Vec<FilterCategory>> {
    create_client_resource(|| async { Extension.filters().await.unwrap_throw() })
}

pub fn use_series_details(series_id: String) -> Resource<Series> {
    create_client_resource(move || {
        let series_id = series_id.clone();
        async move { Extension.get_series_info(&series_id).await.unwrap_throw() }
    })
}

pub fn use_episode_videos(series_id: String, episode_id: String) -> Resource<Vec<SeriesVideo>> {
    create_client_resource(move || {
        let series_id = series_id.clone();
        let episode_id = episode_id.clone();
        async move {
            Extension
                .get_series_videos(&series_id, &episode_id)
                .await
                .unwrap_throw()
        }
    })
}
