#[macro_use]
mod macros;

cfg_not_wasm32! {
    mod semver;
    mod extensions;
    mod host;

    pub use extensions::WasmExtension;
    pub use host::WasmHost;
}

pub mod types;

use anyhow::Result;
use types::{EpisodesPage, FilterCategory, SearchFilter, SeriesPage, SeriesVideo};

pub trait Extension {
    fn filters(&mut self) -> impl Future<Output = Result<Vec<FilterCategory>>> + Send;

    fn search(
        &mut self,
        query: &str,
        page: Option<u16>,
        filters: Vec<SearchFilter>,
    ) -> impl Future<Output = Result<SeriesPage>> + Send;

    fn get_series_episodes(
        &mut self,
        series_id: &str,
    ) -> impl Future<Output = Result<EpisodesPage>> + Send;

    fn get_series_videos(
        &mut self,
        series_id: &str,
        episode_id: &str,
    ) -> impl Future<Output = Result<Vec<SeriesVideo>>> + Send;
}
