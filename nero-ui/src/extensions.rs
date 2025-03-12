use nero_extensions::{
    anyhow::{anyhow, Result},
    types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, SeriesVideo},
    Extension as ExtensionTrait,
};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::UnwrapThrowExt;

use crate::{invoke, invoke_without_args};

pub struct Extension;

impl Extension {
    async fn invoke_and_parse<T>(cmd: &str, args: Option<serde_json::Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let res = match args {
            Some(args_value) => {
                let js_value = to_value(&args_value).unwrap_throw();
                invoke(cmd, js_value)
                    .await
                    .map_err(|e| anyhow!("Error invoking {}: {:?}", cmd, e))?
            }
            None => invoke_without_args(cmd)
                .await
                .map_err(|e| anyhow!("Error invoking {}: {:?}", cmd, e))?,
        };

        from_value::<T>(res).map_err(|e| anyhow!("Error parsing result from {}: {:?}", cmd, e))
    }
}

impl ExtensionTrait for Extension {
    async fn filters(&self) -> Result<Vec<FilterCategory>> {
        Self::invoke_and_parse("get_filters", None).await
    }

    async fn search(
        &self,
        query: &str,
        page: Option<u16>,
        filters: Vec<SearchFilter>,
    ) -> Result<SeriesPage> {
        let args = serde_json::json!({ "query": query, "page": page, "filters": filters });
        Self::invoke_and_parse("search", Some(args)).await
    }

    async fn get_series_info(&self, series_id: &str) -> Result<Series> {
        let args = serde_json::json!({ "series_id": series_id });
        Self::invoke_and_parse("get_series_info", Some(args)).await
    }

    async fn get_series_episodes(
        &self,
        series_id: &str,
        page: Option<u16>,
    ) -> Result<EpisodesPage> {
        let args = serde_json::json!({ "series_id": series_id, "page": page });
        Self::invoke_and_parse("get_series_episodes", Some(args)).await
    }

    async fn get_series_videos(
        &self,
        series_id: &str,
        episode_id: &str,
    ) -> Result<Vec<SeriesVideo>> {
        let args = serde_json::json!({ "series_id": series_id, "episode_id": episode_id });
        Self::invoke_and_parse("get_series_videos", Some(args)).await
    }
}
