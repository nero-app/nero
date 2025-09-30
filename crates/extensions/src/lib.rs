mod extensions;

pub mod types {
    pub use nero_types::*;
}

use anyhow::{Ok, Result, anyhow};
use nero_runtime::{Metadata, semver::SemanticVersion};
use nero_types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, Video};
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::extensions::{AsyncTryIntoWithStore, since_v0_1_0_draft};

#[allow(non_camel_case_types)]
enum ExtensionPre {
    V0_1_0_DRAFT(since_v0_1_0_draft::ExtensionPre<WasmState>),
}

impl ExtensionPre {
    fn engine(&self) -> &Engine {
        match self {
            ExtensionPre::V0_1_0_DRAFT(extension_pre) => extension_pre.engine(),
        }
    }

    async fn instantiate_async(&self, store: &mut Store<WasmState>) -> Result<Extension> {
        match self {
            ExtensionPre::V0_1_0_DRAFT(pre) => {
                let extension = pre.instantiate_async(store).await?;
                Ok(Extension::V0_1_0_DRAFT(extension))
            }
        }
    }
}

#[allow(non_camel_case_types)]
enum Extension {
    V0_1_0_DRAFT(since_v0_1_0_draft::Extension),
}

pub struct WasmExtension {
    extension_pre: ExtensionPre,
    metadata: Metadata,
}

impl nero_runtime::WasmComponent for WasmExtension {
    async fn instantiate_async(
        engine: &Engine,
        version: SemanticVersion,
        component: &Component,
        metadata: Metadata,
    ) -> Result<Self> {
        let mut linker = Linker::new(engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker).unwrap();
        nero_logging::add_to_linker(&mut linker).unwrap();

        let extension_pre = match version {
            v if v >= since_v0_1_0_draft::MIN_VER => Ok(ExtensionPre::V0_1_0_DRAFT(
                since_v0_1_0_draft::ExtensionPre::new(linker.instantiate_pre(component)?)?,
            )),
            _ => Err(anyhow!("unsupported extension version")),
        }?;

        Ok(Self {
            extension_pre,
            metadata,
        })
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl WasmExtension {
    pub async fn filters(&self) -> Result<Vec<FilterCategory>> {
        let mut store = Store::new(self.extension_pre.engine(), WasmState::default());

        match self.extension_pre.instantiate_async(&mut store).await? {
            Extension::V0_1_0_DRAFT(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_filters(&mut store)
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
        let mut store = Store::new(self.extension_pre.engine(), WasmState::default());

        match self.extension_pre.instantiate_async(&mut store).await? {
            Extension::V0_1_0_DRAFT(extension) => {
                let filters = filters.into_iter().map(Into::into).collect::<Vec<_>>();
                let res = extension
                    .nero_extension_extractor()
                    .call_search(&mut store, query, page, &filters)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                res.try_into_with_store(&mut store).await
            }
        }
    }

    pub async fn get_series_info(&self, series_id: &str) -> Result<Series> {
        let mut store = Store::new(self.extension_pre.engine(), WasmState::default());

        match self.extension_pre.instantiate_async(&mut store).await? {
            Extension::V0_1_0_DRAFT(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_info(&mut store, series_id)
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
        let mut store = Store::new(self.extension_pre.engine(), WasmState::default());

        match self.extension_pre.instantiate_async(&mut store).await? {
            Extension::V0_1_0_DRAFT(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_episodes(&mut store, series_id, page)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                res.try_into_with_store(&mut store).await
            }
        }
    }

    pub async fn get_series_videos(&self, series_id: &str, episode_id: &str) -> Result<Vec<Video>> {
        let mut store = Store::new(self.extension_pre.engine(), WasmState::default());

        match self.extension_pre.instantiate_async(&mut store).await? {
            Extension::V0_1_0_DRAFT(extension) => {
                let res = extension
                    .nero_extension_extractor()
                    .call_get_series_videos(&mut store, series_id, episode_id)
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

pub(crate) struct WasmState {
    table: ResourceTable,
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
}

impl WasiView for WasmState {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

impl WasiHttpView for WasmState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl Default for WasmState {
    fn default() -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtx::builder().build(),
            http_ctx: WasiHttpCtx::new(),
        }
    }
}
