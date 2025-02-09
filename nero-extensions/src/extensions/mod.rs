mod since_v0_0_1;

use anyhow::{Ok, Result, anyhow};
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};

use crate::{
    Extension as ExtensionTrait,
    host::WasmState,
    semver::SemanticVersion,
    types::{EpisodesPage, FilterCategory, SearchFilter, SeriesPage, SeriesVideo},
};

enum Extension {
    V001(since_v0_0_1::Extension),
}

pub struct WasmExtension {
    store: Store<WasmState>,
    extension: Extension,
}

impl WasmExtension {
    pub(crate) async fn instantiate_async(
        engine: &Engine,
        mut store: Store<WasmState>,
        version: SemanticVersion,
        component: &Component,
    ) -> Result<Self> {
        let mut linker = Linker::new(engine);
        wasmtime_wasi::add_to_linker_async(&mut linker).unwrap();
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker).unwrap();

        let extension = match version {
            v if v >= since_v0_0_1::MIN_VER => Ok(Extension::V001(
                since_v0_0_1::Extension::instantiate_async(&mut store, component, &linker).await?,
            )),
            _ => Err(anyhow!("unsupported extension version")),
        }?;

        Ok(Self { store, extension })
    }
}

impl ExtensionTrait for WasmExtension {
    async fn filters(&mut self) -> Result<Vec<FilterCategory>> {
        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_filters(&mut self.store)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(res.into_iter().map(Into::into).collect())
            }
        }
    }

    async fn search(
        &mut self,
        query: &str,
        page: Option<u16>,
        filters: Vec<SearchFilter>,
    ) -> Result<SeriesPage> {
        match &self.extension {
            Extension::V001(extension) => {
                let filters = filters.into_iter().map(Into::into).collect::<Vec<_>>();

                let res = extension
                    .nero_extension_extractor()
                    .call_search(&mut self.store, query, page, &filters)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(res.into())
            }
        }
    }

    async fn get_series_episodes(&mut self, series_id: &str) -> Result<EpisodesPage> {
        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_episodes(&mut self.store, series_id)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(res.into())
            }
        }
    }

    async fn get_series_videos(
        &mut self,
        series_id: &str,
        episode_id: &str,
    ) -> Result<Vec<SeriesVideo>> {
        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_videos(&mut self.store, series_id, episode_id)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                let videos = res
                    .into_iter()
                    .map(|v| v.into_crate_video(&mut self.store))
                    .collect::<Result<_>>()?;

                Ok(videos)
            }
        }
    }
}
