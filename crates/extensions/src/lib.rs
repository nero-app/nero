mod extensions;
pub mod types;

use anyhow::{Result, anyhow};
use nero_wasm_host::{Metadata, semver::SemanticVersion};
use wasmtime::{Engine, Store, component::Component};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::{
    extensions::{AsyncTryIntoWithStore, Extension, ExtensionPre, since_v0_1_0_draft},
    types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, Video},
};

struct WasmState {
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

pub struct WasmExtension {
    extension_pre: ExtensionPre,
    metadata: Metadata,
}

impl nero_wasm_host::WasmComponent for WasmExtension {
    async fn instantiate_async(
        engine: &Engine,
        version: SemanticVersion,
        component: &Component,
        metadata: Metadata,
    ) -> Result<Self> {
        let extension_pre = match version {
            v if v >= since_v0_1_0_draft::MIN_VER => {
                let linker = since_v0_1_0_draft::linker(engine)?;
                let pre = linker.instantiate_pre(component)?;
                Ok(ExtensionPre::V0_1_0_DRAFT(
                    since_v0_1_0_draft::ExtensionPre::new(pre)?,
                ))
            }
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
