mod keyvalue;
mod processors;
pub mod server;

use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, LazyLock},
};

use anyhow::{Result, anyhow};
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::header::{HeaderName, HeaderValue};
use keyvalue::WasiKeyValueCtx;
use nero_runtime::{Metadata, semver::SemanticVersion};
use nero_types::HttpResource;
use tokio::sync::RwLock;
use url::Url;
use wasmtime::{
    Engine, Store,
    component::{Component, HasSelf, Linker, Resource},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtx, WasiView},
};
use wasmtime_wasi_http::{
    WasiHttpCtx, WasiHttpView,
    bindings::{
        exports::wasi::http::incoming_handler::{IncomingRequest, ResponseOutparam},
        http::types::Scheme,
    },
    types::HostIncomingRequest,
};

use crate::{processors::since_v0_1_0_draft, server::SERVER_ADDR};

pub static KEYVALUE_STORE: LazyLock<Arc<RwLock<HashMap<String, Vec<u8>>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

#[allow(non_camel_case_types)]
#[derive(Clone)]
enum ProcessorPre {
    V0_1_0_DRAFT(since_v0_1_0_draft::ProcessorPre<WasmState>),
}

impl ProcessorPre {
    fn engine(&self) -> &Engine {
        match self {
            ProcessorPre::V0_1_0_DRAFT(processor_pre) => processor_pre.engine(),
        }
    }

    async fn instantiate_async(&self, store: &mut Store<WasmState>) -> Result<Processor> {
        match self {
            ProcessorPre::V0_1_0_DRAFT(pre) => {
                let processor = pre.instantiate_async(store).await?;
                Ok(Processor::V0_1_0_DRAFT(processor))
            }
        }
    }
}

#[allow(non_camel_case_types)]
enum Processor {
    V0_1_0_DRAFT(since_v0_1_0_draft::Processor),
}

impl Processor {
    async fn resolve_resource(
        &self,
        store: &mut Store<WasmState>,
        incoming: Resource<HostIncomingRequest>,
    ) -> Result<String> {
        match self {
            Processor::V0_1_0_DRAFT(processor) => {
                processor.call_resolve_resource(store, incoming).await
            }
        }
    }

    async fn handle_incoming_request(
        &self,
        store: &mut Store<WasmState>,
        req: Resource<IncomingRequest>,
        out: Resource<ResponseOutparam>,
    ) -> Result<()> {
        match self {
            Processor::V0_1_0_DRAFT(processor) => {
                processor
                    .wasi_http_incoming_handler()
                    .call_handle(store, req, out)
                    .await
            }
        }
    }
}

pub struct WasmProcessor {
    processor_pre: ProcessorPre,
    metadata: Metadata,
}

impl nero_runtime::WasmComponent for WasmProcessor {
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
        keyvalue::keyvalue::store::add_to_linker::<_, HasSelf<_>>(&mut linker, |s| s).unwrap();

        let processor_pre = match version {
            v if v >= since_v0_1_0_draft::MIN_VER => Ok(ProcessorPre::V0_1_0_DRAFT(
                since_v0_1_0_draft::ProcessorPre::new(linker.instantiate_pre(component)?)?,
            )),
            _ => Err(anyhow!("unsupported extension version")),
        }?;

        Ok(Self {
            processor_pre,
            metadata,
        })
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl WasmProcessor {
    pub async fn resolve_resource(&self, resource: HttpResource) -> Result<Url> {
        let engine = self.processor_pre.engine();
        let mut store = Store::new(engine, WasmState::default());

        let method = match resource.method {
            nero_types::Method::Get => hyper::Method::GET,
            nero_types::Method::Head => hyper::Method::HEAD,
            nero_types::Method::Post => hyper::Method::POST,
            nero_types::Method::Put => hyper::Method::PUT,
            nero_types::Method::Delete => hyper::Method::DELETE,
            nero_types::Method::Connect => hyper::Method::CONNECT,
            nero_types::Method::Options => hyper::Method::OPTIONS,
            nero_types::Method::Trace => hyper::Method::TRACE,
            nero_types::Method::Patch => hyper::Method::PATCH,
            nero_types::Method::Other(other) => hyper::Method::from_bytes(other.as_bytes())?,
        };
        let mut req_builder = hyper::Request::builder()
            .method(method)
            .uri(resource.url.as_str());
        for (key, value) in resource.headers {
            let header_name = HeaderName::from_str(&key)?;
            let header_value = HeaderValue::from_str(&value)?;

            req_builder = req_builder.header(header_name, header_value);
        }
        let body =
            Full::new(Bytes::from(resource.body.unwrap_or_default())).map_err(|_| unreachable!());
        let req = req_builder.body(body)?;
        let incoming = store.data_mut().new_incoming_request(Scheme::Http, req)?;

        let processor = self.processor_pre.instantiate_async(&mut store).await?;
        let res = processor.resolve_resource(&mut store, incoming).await?;

        let addr = SERVER_ADDR
            .get()
            .ok_or_else(|| anyhow!("Server address not set"))?;

        Url::parse(&format!("http://{addr}/{res}"))
            .map_err(|e| anyhow!("Failed to parse URL: {}", e))
    }
}

struct WasmState {
    table: ResourceTable,
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
    wasi_keyvalue_ctx: WasiKeyValueCtx,
}

impl Default for WasmState {
    fn default() -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtx::builder().build(),
            http_ctx: WasiHttpCtx::new(),
            wasi_keyvalue_ctx: WasiKeyValueCtx::new(KEYVALUE_STORE.clone()),
        }
    }
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
