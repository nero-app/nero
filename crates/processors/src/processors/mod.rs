use anyhow::Result;
use wasmtime::{Engine, Store, component::Resource};
use wasmtime_wasi_http::bindings::exports::wasi::http::incoming_handler::{
    IncomingRequest, ResponseOutparam,
};

use crate::WasmState;

pub mod since_v0_1_0_draft;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub(crate) enum ProcessorPre {
    V0_1_0_DRAFT(since_v0_1_0_draft::ProcessorPre<WasmState>),
}

impl ProcessorPre {
    pub fn engine(&self) -> &Engine {
        match self {
            ProcessorPre::V0_1_0_DRAFT(processor_pre) => processor_pre.engine(),
        }
    }

    pub async fn instantiate_async(&self, store: &mut Store<WasmState>) -> Result<Processor> {
        match self {
            ProcessorPre::V0_1_0_DRAFT(pre) => {
                let processor = pre.instantiate_async(store).await?;
                Ok(Processor::V0_1_0_DRAFT(processor))
            }
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Processor {
    V0_1_0_DRAFT(since_v0_1_0_draft::Processor),
}

impl Processor {
    pub async fn handle_incoming_request(
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
