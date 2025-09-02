use anyhow::Result;
use wasmtime::Store;

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
