use std::path::PathBuf;

use anyhow::Result;
use tokio::fs::{read, read_dir};
use wasm_metadata::{Metadata, Payload};

use crate::{WasmExtension, WasmHost};

const WASM_EXTENSION: &str = "wasm";

pub struct ExtensionManager {
    extensions_dir: PathBuf,
    wasm_host: WasmHost,
}

impl ExtensionManager {
    pub fn new(extensions_dir: impl Into<PathBuf>) -> Result<Self> {
        let extensions_dir = extensions_dir.into();
        if !extensions_dir.exists() {
            std::fs::create_dir_all(&extensions_dir)?
        }

        Ok(Self {
            extensions_dir,
            wasm_host: WasmHost::default(),
        })
    }

    pub async fn get_available_extensions(&self) -> Result<Vec<(String, Metadata)>> {
        let mut extensions = vec![];
        let mut entries = read_dir(&self.extensions_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file()
                && path.extension().and_then(|ext| ext.to_str()) == Some(WASM_EXTENSION)
            {
                let wasm_data = read(&path).await?;
                let payload = Payload::from_binary(&wasm_data.to_vec())?;
                extensions.push(match payload {
                    Payload::Component { metadata, .. } => {
                        let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                        (file_name, metadata)
                    }
                    _ => continue,
                });
            }
        }
        Ok(extensions)
    }

    pub async fn load_extension_async<T: WasmExtension>(&self, file_name: &str) -> Result<T> {
        let res = self
            .wasm_host
            .load_extension_async(
                self.extensions_dir
                    .join(format!("{file_name}.{WASM_EXTENSION}")),
            )
            .await?;
        Ok(res)
    }

    pub async fn delete_extension_async(&self, file_name: &str) -> Result<()> {
        let path = self
            .extensions_dir
            .join(format!("{file_name}.{WASM_EXTENSION}"));
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
}
