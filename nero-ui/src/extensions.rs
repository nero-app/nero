use nero_extensions::{
    anyhow::Result,
    types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, SeriesVideo},
    Extension as ExtensionTrait,
};

use crate::utils::invoke_and_parse;

pub struct Extension;

impl ExtensionTrait for Extension {
    async fn filters(&self) -> Result<Vec<FilterCategory>> {
        invoke_and_parse("get_filters", None).await
    }

    async fn search(
        &self,
        query: &str,
        page: Option<u16>,
        filters: Vec<SearchFilter>,
    ) -> Result<SeriesPage> {
        let args = serde_json::json!({ "query": query, "page": page, "filters": filters });
        invoke_and_parse("search", Some(args)).await
    }

    async fn get_series_info(&self, series_id: &str) -> Result<Series> {
        let args = serde_json::json!({ "series_id": series_id });
        invoke_and_parse("get_series_info", Some(args)).await
    }

    async fn get_series_episodes(
        &self,
        series_id: &str,
        page: Option<u16>,
    ) -> Result<EpisodesPage> {
        let args = serde_json::json!({ "series_id": series_id, "page": page });
        invoke_and_parse("get_series_episodes", Some(args)).await
    }

    async fn get_series_videos(
        &self,
        series_id: &str,
        episode_id: &str,
    ) -> Result<Vec<SeriesVideo>> {
        let args = serde_json::json!({ "series_id": series_id, "episode_id": episode_id });
        invoke_and_parse("get_series_videos", Some(args)).await
    }
}
