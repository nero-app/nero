mod since_v0_0_1;

use anyhow::{Ok, Result, anyhow};
use tokio::sync::Mutex;
use wasm_metadata::Metadata;
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};
use wasmtime_wasi::p2::IoImpl;

use crate::{
    Extension as ExtensionTrait,
    host::WasmState,
    logging::{self, WasiLogging, WasiLoggingImpl},
    semver::SemanticVersion,
    types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, Video},
};

enum Extension {
    V001(since_v0_0_1::Extension),
}

pub struct WasmExtension {
    store: Mutex<Store<WasmState>>,
    extension: Extension,
    metadata: Metadata,
}

impl WasmExtension {
    pub(crate) async fn instantiate_async(
        engine: &Engine,
        mut store: Store<WasmState>,
        version: SemanticVersion,
        component: &Component,
        metadata: Metadata,
    ) -> Result<Self> {
        let mut linker = Linker::new(engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker).unwrap();
        logging::add_to_linker::<_, WasiLogging<WasmState>>(&mut linker, |s| {
            WasiLoggingImpl(IoImpl(s))
        })
        .unwrap();

        let extension = match version {
            v if v >= since_v0_0_1::MIN_VER => Ok(Extension::V001(
                since_v0_0_1::Extension::instantiate_async(&mut store, component, &linker).await?,
            )),
            _ => Err(anyhow!("unsupported extension version")),
        }?;

        Ok(Self {
            store: Mutex::new(store),
            extension,
            metadata,
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl ExtensionTrait for WasmExtension {
    async fn filters(&self) -> Result<Vec<FilterCategory>> {
        let mut store = self.store.lock().await;

        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_filters(&mut *store)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(res.into_iter().map(Into::into).collect())
            }
        }
    }

    async fn search(
        &self,
        query: &str,
        page: Option<u16>,
        filters: Vec<SearchFilter>,
    ) -> Result<SeriesPage> {
        let mut store = self.store.lock().await;

        match &self.extension {
            Extension::V001(extension) => {
                let filters = filters.into_iter().map(Into::into).collect::<Vec<_>>();
                let res = extension
                    .nero_extension_extractor()
                    .call_search(&mut *store, query, page, &filters)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(res.into())
            }
        }
    }

    async fn get_series_info(&self, series_id: &str) -> Result<Series> {
        let mut store = self.store.lock().await;

        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_info(&mut *store, series_id)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(res.into())
            }
        }
    }

    async fn get_series_episodes(
        &self,
        series_id: &str,
        page: Option<u16>,
    ) -> Result<EpisodesPage> {
        let mut store = self.store.lock().await;

        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_episodes(&mut *store, series_id, page)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(res.into())
            }
        }
    }

    async fn get_series_videos(&self, series_id: &str, episode_id: &str) -> Result<Vec<Video>> {
        let mut store = self.store.lock().await;

        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_videos(&mut *store, series_id, episode_id)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                let videos = res
                    .into_iter()
                    .map(|v| v.into_crate_video(&mut store))
                    .collect::<Result<_>>()?;

                Ok(videos)
            }
        }
    }
}
