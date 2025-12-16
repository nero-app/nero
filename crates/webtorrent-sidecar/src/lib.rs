use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use nodejs_installer::NodeJs;
use semver::{Version, VersionReq};
use tokio::process::Command;

use crate::command::WebTorrentCommand;

pub mod command;

const NPM_WEBTORRENT_REGISTRY: &str = "https://registry.npmjs.org/webtorrent-cli";

const SUPPORTED_NODE_VERSIONS: &str = ">=17.0.0, <23.0.0";
const SUPPORTED_WEBTORRENT_VERSIONS: &str = ">=5.0.0, <6.0.0";
const WEBTORRENT_CMD_PATH: &str = "node_modules/webtorrent-cli/bin/cmd.js";

#[derive(Debug, Clone)]
pub struct WebTorrent {
    node_path: PathBuf,
    webtorrent_path: PathBuf,
    version: Version,
}

impl WebTorrent {
    pub async fn from_paths(node_path: PathBuf, webtorrent_path: PathBuf) -> Result<Self> {
        let version = Self::get_webtorrent_version(&node_path, &webtorrent_path).await?;

        Ok(Self {
            node_path,
            webtorrent_path,
            version,
        })
    }

    pub async fn from_local(
        node_js: &NodeJs,
        cache_dir: &PathBuf,
        version_req: &VersionReq,
    ) -> Result<Self> {
        let supported_node = VersionReq::parse(SUPPORTED_NODE_VERSIONS)?;
        if !supported_node.matches(node_js.node_version()) {
            bail!(
                "Node.js version {} is not compatible. Required: {}",
                node_js.node_version(),
                SUPPORTED_NODE_VERSIONS
            )
        }

        let mut entries = tokio::fs::read_dir(cache_dir).await?;

        let mut matching_versions = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let dir_name = entry.file_name();
            let version = match Version::parse(&dir_name.to_string_lossy()) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let webtorrent_cmd = entry.path().join(WEBTORRENT_CMD_PATH);
            if !webtorrent_cmd.exists() {
                continue;
            }

            let supported_versions = VersionReq::parse(SUPPORTED_WEBTORRENT_VERSIONS)?;

            if version_req.matches(&version) && supported_versions.matches(&version) {
                matching_versions.push((version, webtorrent_cmd));
            }
        }

        matching_versions
            .into_iter()
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(version, webtorrent_path)| Self {
                node_path: node_js.node().to_path_buf(),
                webtorrent_path,
                version,
            })
            .context("No matching local installation found")
    }

    pub async fn from_system(node_js: &NodeJs, version_req: &VersionReq) -> Result<Self> {
        let supported_node = VersionReq::parse(SUPPORTED_NODE_VERSIONS)?;
        if !supported_node.matches(node_js.node_version()) {
            bail!(
                "Node.js version {} is not compatible. Required: {}",
                node_js.node_version(),
                SUPPORTED_NODE_VERSIONS
            )
        }

        let webtorrent_paths = which::which_all("webtorrent")?;

        let supported_versions = VersionReq::parse(SUPPORTED_WEBTORRENT_VERSIONS)?;

        for webtorrent_path in webtorrent_paths {
            if !webtorrent_path.exists() {
                continue;
            }

            if let Ok(version) =
                Self::get_webtorrent_version(node_js.node(), &webtorrent_path).await
                && version_req.matches(&version)
                && supported_versions.matches(&version)
            {
                return Ok(Self {
                    node_path: node_js.node().to_path_buf(),
                    webtorrent_path,
                    version,
                });
            }
        }

        bail!("No compatible webtorrent-cli installation found in system (required: {version_req})")
    }

    pub async fn download(
        node_js: &NodeJs,
        install_path: &PathBuf,
        version_req: &VersionReq,
    ) -> Result<Self> {
        let supported_node = VersionReq::parse(SUPPORTED_NODE_VERSIONS)?;
        if !supported_node.matches(node_js.node_version()) {
            bail!(
                "Node.js version {} is not compatible. Required: {}",
                node_js.node_version(),
                SUPPORTED_NODE_VERSIONS
            )
        }

        tokio::fs::create_dir_all(install_path).await?;

        let resolved_version = Self::resolve_remote_version(version_req).await?;

        let supported_webtorrent = VersionReq::parse(SUPPORTED_WEBTORRENT_VERSIONS)?;
        if !supported_webtorrent.matches(&resolved_version) {
            bail!(
                "Resolved webtorrent-cli version {} (from requirement '{}') is not compatible. Required: {}",
                resolved_version,
                version_req,
                SUPPORTED_WEBTORRENT_VERSIONS
            )
        }

        let version_path = install_path.join(resolved_version.to_string());
        tokio::fs::create_dir_all(&version_path).await?;

        let output = Command::new(node_js.npm())
            .current_dir(&version_path)
            .arg("install")
            .arg("--prefix")
            .arg(&version_path)
            .arg(format!("webtorrent-cli@{}", resolved_version))
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            bail!("Failed to install webtorrent-cli:\nSTDOUT: {stdout}\nSTDERR: {stderr}")
        }

        let webtorrent_cmd = version_path.join(WEBTORRENT_CMD_PATH);

        if !webtorrent_cmd.exists() {
            bail!("webtorrent-cli was installed but could not be found")
        }

        Ok(Self {
            node_path: node_js.node().to_path_buf(),
            webtorrent_path: webtorrent_cmd,
            version: resolved_version,
        })
    }

    async fn resolve_remote_version(version_req: &VersionReq) -> Result<Version> {
        let response = reqwest::get(NPM_WEBTORRENT_REGISTRY)
            .await?
            .json::<serde_json::Value>()
            .await?;

        let versions = response
            .get("versions")
            .and_then(|v| v.as_object())
            .context("Invalid response format from npm registry")?;

        let mut available_versions = versions
            .keys()
            .filter_map(|v| Version::parse(v).ok())
            .collect::<Vec<_>>();

        available_versions.sort_by(|a, b| b.cmp(a));

        let matching_version = available_versions
            .into_iter()
            .find(|version| version_req.matches(version))
            .context(format!(
                "No version matching '{version_req}' found in npm registry",
            ))?;

        Ok(matching_version)
    }

    async fn get_webtorrent_version(node_path: &Path, webtorrent_path: &Path) -> Result<Version> {
        let output = Command::new(node_path)
            .arg(webtorrent_path)
            .arg("--version")
            .output()
            .await?;

        if !output.status.success() {
            bail!("Failed to get webtorrent-cli version")
        }

        let version_str = String::from_utf8_lossy(&output.stdout);
        let version_part = version_str
            .split_whitespace()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid version format"))?;

        Ok(Version::parse(version_part)?)
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn webtorrent_path(&self) -> &Path {
        &self.webtorrent_path
    }

    pub fn command(&self) -> WebTorrentCommand {
        WebTorrentCommand::new(&self.node_path, &self.webtorrent_path)
    }
}
