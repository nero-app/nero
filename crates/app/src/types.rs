use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NodeJsConfig {
    Custom {
        node: PathBuf,
        npm: PathBuf,
    },
    #[serde(rename_all = "camelCase")]
    AutoDetect {
        auto_download: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WebTorrentConfig {
    Custom {
        webtorrent: PathBuf,
    },
    #[serde(rename_all = "camelCase")]
    AutoDetect {
        auto_download: bool,
    },
}
