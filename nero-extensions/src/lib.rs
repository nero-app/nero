mod extensions;
mod logging;

pub mod types {
    pub use nero_types::*;
}

use anyhow::{Ok, Result, anyhow};
use nero_runtime::{Metadata, semver::SemanticVersion};
use nero_types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, Video};
use tokio::sync::Mutex;
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoImpl, IoView, WasiCtx, WasiView},
};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::{
    extensions::{AsyncTryIntoWithStore, since_v0_1_0_draft},
    logging::{WasiLogging, WasiLoggingImpl},
};

pub(crate) struct WasmState {
    table: ResourceTable,
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
}

impl IoView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for WasmState {
    fn ctx(&mut self) -> &mut wasmtime_wasi::p2::WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for WasmState {
    fn ctx(&mut self) -> &mut wasmtime_wasi_http::WasiHttpCtx {
        &mut self.http_ctx
    }
}

enum Extension {
    V001(since_v0_1_0_draft::Extension),
}

pub struct WasmExtension {
    store: Mutex<Store<WasmState>>,
    extension: Extension,
    metadata: Metadata,
}

impl nero_runtime::WasmExtension for WasmExtension {
    async fn instantiate_async(
        engine: &Engine,
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

        let mut store = Store::new(
            engine,
            WasmState {
                table: ResourceTable::new(),
                ctx: WasiCtx::builder().build(),
                http_ctx: WasiHttpCtx::new(),
            },
        );

        let extension = match version {
            v if v >= since_v0_1_0_draft::MIN_VER => Ok(Extension::V001(
                since_v0_1_0_draft::Extension::instantiate_async(&mut store, component, &linker)
                    .await?,
            )),
            _ => Err(anyhow!("unsupported extension version")),
        }?;

        Ok(Self {
            store: Mutex::new(store),
            extension,
            metadata,
        })
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl WasmExtension {
    pub async fn filters(&self) -> Result<Vec<FilterCategory>> {
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

    pub async fn search(
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

                res.try_into_with_store(&mut store).await
            }
        }
    }

    pub async fn get_series_info(&self, series_id: &str) -> Result<Series> {
        let mut store = self.store.lock().await;

        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_info(&mut *store, series_id)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                res.try_into_with_store(&mut store).await
            }
        }
    }

    pub async fn get_series_episodes(
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

                res.try_into_with_store(&mut store).await
            }
        }
    }

    pub async fn get_series_videos(&self, series_id: &str, episode_id: &str) -> Result<Vec<Video>> {
        let mut store = self.store.lock().await;

        match &self.extension {
            Extension::V001(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_videos(&mut *store, series_id, episode_id)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                let mut items = Vec::new();
                for video in res {
                    items.push(video.try_into_with_store(&mut store).await?);
                }
                Ok(items)
            }
        }
    }
}
