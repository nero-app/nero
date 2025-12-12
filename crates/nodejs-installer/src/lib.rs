mod archive;

use std::{
    env::consts::EXE_EXTENSION,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use futures::TryStreamExt;
use is_executable::IsExecutable;
use semver::{Version, VersionReq};
use tokio::process::Command;
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[derive(Debug, Clone)]
pub struct NodeJs {
    node: PathBuf,
    npm: PathBuf,
    version: Version,
}

impl NodeJs {
    pub async fn auto_download(path: PathBuf, version_req: &VersionReq) -> Result<Self> {
        tokio::fs::create_dir_all(&path).await?;

        if let Ok(node) = Self::find_local_installation(&path, version_req).await {
            return Ok(node);
        }

        if let Some(node) = Self::find_system_installation(version_req).await? {
            return Ok(node);
        }

        let resolved_version = Self::resolve_remote_version(version_req).await?;
        Self::download(&path, resolved_version).await
    }

    async fn find_local_installation(
        install_path: &PathBuf,
        version_req: &VersionReq,
    ) -> Result<Self> {
        let mut entries = tokio::fs::read_dir(install_path).await?;

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

            if version_req.matches(&version) {
                matching_versions.push((version, entry.path()));
            }
        }

        matching_versions
            .into_iter()
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(version, path)| Self::from_path_and_version(path, version))
            .context("No matching installed version found")
    }

    async fn find_system_installation(version_req: &VersionReq) -> Result<Option<Self>> {
        let node_paths = match which::which_all("node") {
            Ok(paths) => paths,
            Err(_) => return Ok(None),
        };

        for node_path in node_paths {
            let bin_dir = match node_path.parent() {
                Some(p) => p,
                None => continue,
            };

            let npm_path = ["npm", "npm.cmd", "npm.bat"]
                .iter()
                .map(|name| bin_dir.join(name))
                .find(|candidate| {
                    candidate.try_exists().unwrap_or(false) && candidate.is_executable()
                });

            let Some(npm_path) = npm_path else {
                continue;
            };

            if let Ok(version) = Self::get_version_from_node(&node_path).await
                && version_req.matches(&version)
            {
                let node = Self {
                    node: node_path.to_path_buf(),
                    npm: npm_path,
                    version,
                };
                return Ok(Some(node));
            }
        }

        Ok(None)
    }

    async fn resolve_remote_version(version_req: &VersionReq) -> Result<Version> {
        let response_json = reqwest::get("https://nodejs.org/dist/index.json")
            .await?
            .json::<serde_json::Value>()
            .await?;

        let available_versions: Vec<Version> = response_json
            .as_array()
            .context("Invalid response format from nodejs.org")?
            .iter()
            .filter_map(|item| item.get("version"))
            .filter_map(|v| v.as_str())
            .filter_map(|s| {
                let s = s.strip_prefix('v').unwrap_or(s);
                Version::parse(s).ok()
            })
            .collect();

        let matching_version = available_versions
            .into_iter()
            .find(|version| version_req.matches(version))
            .context(format!(
                "Requested version '{version_req}' not found remotely"
            ))?;

        Ok(matching_version)
    }

    async fn download(install_path: &PathBuf, version: Version) -> Result<Self> {
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "win",
            _ => bail!("Unsupported OS"),
        };
        let arch = match std::env::consts::ARCH {
            "aarch64" => "arm64",
            "x86" => "x86",
            "x86_64" => "x64",
            _ => bail!("Unsupported architecture"),
        };

        if os == "darwin" && arch == "arm64" && version.major < 16 {
            bail!("Unsupported Node.js version {version} on this architecture");
        }

        let ext = match std::env::consts::OS {
            "windows" => "zip",
            _ => "tar.xz",
        };

        let filename = format!("node-v{version}-{os}-{arch}.{ext}");
        let url = format!("https://nodejs.org/dist/v{version}/{filename}");
        let target_path = install_path.join(version.to_string());

        let response = reqwest::get(&url).await?;
        if !response.status().is_success() {
            bail!("Failed to download file from {url}: {}", response.status());
        }

        let reader = response
            .bytes_stream()
            .map_err(std::io::Error::other)
            .into_async_read()
            .compat();

        let temp_dir = tempfile::tempdir_in(install_path)?;

        archive::extract(reader, Path::new(&filename), temp_dir.path()).await?;

        let mut entries_stream = tokio::fs::read_dir(temp_dir.path()).await?;
        let Some(first_entry) = entries_stream.next_entry().await? else {
            bail!("The extracted archive is empty");
        };
        if entries_stream.next_entry().await?.is_some() {
            bail!("Expected single directory in archive, found multiple entries");
        }

        let extracted_path = first_entry.path();

        if target_path.is_dir() {
            tokio::fs::remove_dir_all(&target_path).await?;
        }

        tokio::fs::rename(extracted_path, &target_path).await?;

        Ok(Self::from_path_and_version(target_path, version))
    }

    async fn get_version_from_node(node_path: &Path) -> Result<Version> {
        let output = Command::new(node_path)
            .arg("-p")
            .arg("process.version")
            .output()
            .await?;

        let version_str = String::from_utf8_lossy(&output.stdout);
        let cleaned_version = version_str
            .trim()
            .strip_prefix('v')
            .unwrap_or(version_str.trim());

        Ok(Version::parse(cleaned_version)?)
    }

    fn from_path_and_version(install_path: PathBuf, version: Version) -> Self {
        let bin_dir = match std::env::consts::OS {
            "windows" => install_path,
            _ => install_path.join("bin"),
        };
        let node = bin_dir.join("node").with_extension(EXE_EXTENSION);
        let npm = bin_dir
            .join("npm")
            .with_extension(match std::env::consts::OS {
                "windows" => "cmd",
                _ => "",
            });

        Self { node, npm, version }
    }

    pub fn node(&self) -> &Path {
        &self.node
    }

    pub fn npm(&self) -> &Path {
        &self.npm
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}
