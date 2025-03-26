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
pub use anyhow;
pub use url;
pub use wasm_metadata::Metadata;

use anyhow::Result;
use types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, SeriesVideo};

pub trait Extension {
    fn filters(&self) -> impl Future<Output = Result<Vec<FilterCategory>>>;

    fn search(
        &self,
        query: &str,
        page: Option<u16>,
        filters: Vec<SearchFilter>,
    ) -> impl Future<Output = Result<SeriesPage>>;

    fn get_series_info(&self, series_id: &str) -> impl Future<Output = Result<Series>>;

    fn get_series_episodes(
        &self,
        series_id: &str,
        page: Option<u16>,
    ) -> impl Future<Output = Result<EpisodesPage>>;

    fn get_series_videos(
        &self,
        series_id: &str,
        episode_id: &str,
    ) -> impl Future<Output = Result<Vec<SeriesVideo>>>;
}
