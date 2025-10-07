use anyhow::Result;
use wasmtime::{Engine, Store};

use crate::WasmState;

pub mod since_v0_1_0_draft;

pub(super) trait AsyncTryFromWithStore<T>: Sized {
    async fn try_from_with_store(value: T, store: &mut Store<WasmState>) -> Result<Self>;
}

pub(super) trait AsyncTryIntoWithStore<T> {
    async fn try_into_with_store(self, store: &mut Store<WasmState>) -> Result<T>;
}

impl<T, U> AsyncTryIntoWithStore<U> for T
where
    U: AsyncTryFromWithStore<T>,
{
    async fn try_into_with_store(self, store: &mut Store<WasmState>) -> Result<U> {
        U::try_from_with_store(self, store).await
    }
}

#[allow(non_camel_case_types)]
pub enum ExtensionPre {
    V0_1_0_DRAFT(since_v0_1_0_draft::ExtensionPre<WasmState>),
}

impl ExtensionPre {
    pub fn engine(&self) -> &Engine {
        match self {
            ExtensionPre::V0_1_0_DRAFT(extension_pre) => extension_pre.engine(),
        }
    }

    pub async fn instantiate_async(&self, store: &mut Store<WasmState>) -> Result<Extension> {
        match self {
            ExtensionPre::V0_1_0_DRAFT(pre) => {
                let extension = pre.instantiate_async(store).await?;
                Ok(Extension::V0_1_0_DRAFT(extension))
            }
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Extension {
    V0_1_0_DRAFT(since_v0_1_0_draft::Extension),
}
