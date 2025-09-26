use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use tokio::sync::RwLock;
use wasmtime::component::{Resource, ResourceTableError};

use crate::WasmState;

pub use self::generated::wasi::*;

mod generated {
    wasmtime::component::bindgen!({
        path: "./wit/v0.1.0-draft",
        world: "wasi:keyvalue/imports",
        imports: { default: async | trappable },
        with: {
            "wasi:keyvalue/store/bucket": crate::keyvalue::Bucket
        },
        trappable_error_type: {
            "wasi:keyvalue/store/error" => crate::keyvalue::Error,
        },
    });
}

pub enum Error {
    NoSuchStore,
    #[allow(dead_code)]
    AccessDenied,
    Other(String),
}

impl From<ResourceTableError> for Error {
    fn from(err: ResourceTableError) -> Self {
        Self::Other(err.to_string())
    }
}

pub struct Bucket {
    in_memory_data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

pub struct WasiKeyValueCtx {
    in_memory_data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl WasiKeyValueCtx {
    pub fn new(store: Arc<RwLock<HashMap<String, Vec<u8>>>>) -> Self {
        Self {
            in_memory_data: store,
        }
    }
}

impl keyvalue::store::Host for WasmState {
    async fn open(&mut self, identifier: String) -> Result<Resource<Bucket>, Error> {
        match identifier.as_str() {
            "" => Ok(self.table.push(Bucket {
                in_memory_data: self.wasi_keyvalue_ctx.in_memory_data.clone(),
            })?),
            _ => Err(Error::NoSuchStore),
        }
    }

    fn convert_error(&mut self, err: Error) -> Result<keyvalue::store::Error> {
        match err {
            Error::NoSuchStore => Ok(keyvalue::store::Error::NoSuchStore),
            Error::AccessDenied => Ok(keyvalue::store::Error::AccessDenied),
            Error::Other(e) => Ok(keyvalue::store::Error::Other(e)),
        }
    }
}

impl keyvalue::store::HostBucket for WasmState {
    async fn get(
        &mut self,
        bucket: Resource<Bucket>,
        key: String,
    ) -> Result<Option<Vec<u8>>, Error> {
        let bucket = self.table.get(&bucket)?;
        let data = bucket.in_memory_data.read().await;
        Ok(data.get(&key).cloned())
    }

    async fn set(
        &mut self,
        bucket: Resource<Bucket>,
        key: String,
        value: Vec<u8>,
    ) -> Result<(), Error> {
        let bucket = self.table.get(&bucket)?;
        let mut data = bucket.in_memory_data.write().await;
        data.insert(key, value);
        Ok(())
    }

    async fn delete(&mut self, bucket: Resource<Bucket>, key: String) -> Result<(), Error> {
        let bucket = self.table.get(&bucket)?;
        let mut data = bucket.in_memory_data.write().await;
        data.remove(&key);
        Ok(())
    }

    async fn exists(&mut self, bucket: Resource<Bucket>, key: String) -> Result<bool, Error> {
        let bucket = self.table.get(&bucket)?;
        let data = bucket.in_memory_data.read().await;
        Ok(data.contains_key(&key))
    }

    async fn list_keys(
        &mut self,
        bucket: Resource<Bucket>,
        cursor: Option<u64>,
    ) -> Result<keyvalue::store::KeyResponse, Error> {
        let bucket = self.table.get(&bucket)?;
        let data = bucket.in_memory_data.read().await;
        let keys: Vec<String> = data.keys().cloned().collect();
        let cursor = cursor.unwrap_or(0) as usize;
        let keys_slice = &keys[cursor..];
        Ok(keyvalue::store::KeyResponse {
            keys: keys_slice.to_vec(),
            cursor: None,
        })
    }

    async fn drop(&mut self, bucket: Resource<Bucket>) -> Result<()> {
        self.table.delete(bucket)?;
        Ok(())
    }
}
