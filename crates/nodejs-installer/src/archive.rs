use std::path::Path;
use std::path::{Component, PathBuf};

use anyhow::{Result, bail};
use tokio::io::{AsyncRead, BufReader};

fn enclosed_name(file_name: &str) -> Option<PathBuf> {
    if file_name.contains('\0') {
        return None;
    }
    let path = PathBuf::from(file_name);
    let mut depth = 0usize;
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => return None,
            Component::ParentDir => depth = depth.checked_sub(1)?,
            Component::Normal(_) => depth += 1,
            Component::CurDir => (),
        }
    }
    Some(path)
}

#[cfg(windows)]
async fn unzip<R: AsyncRead + Unpin>(reader: R, target: &Path) -> Result<()> {
    use async_zip::base::read::stream::ZipFileReader;

    let mut reader = BufReader::new(reader);
    let mut zip = ZipFileReader::with_tokio(&mut reader);

    while let Some(mut entry) = zip.next_with_entry().await? {
        let path_str = entry.reader().entry().filename().as_str()?;

        let Some(safe_path) = enclosed_name(path_str) else {
            zip = entry.skip().await?;
            continue;
        };

        let path = target.join(safe_path);

        if entry.reader().entry().dir()? {
            tokio::fs::create_dir_all(&path).await?;
        } else {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            let mut file = tokio::fs::File::create(&path).await?;
            let mut compat_reader = entry.reader_mut().compat();
            tokio::io::copy(&mut compat_reader, &mut file).await?;
        }

        zip = entry.skip().await?;
    }

    Ok(())
}

#[cfg(not(windows))]
async fn untar_xz<R: AsyncRead + Unpin>(reader: R, target: &Path) -> Result<()> {
    use async_compression::tokio::bufread::XzDecoder;
    use async_tar::Archive;
    use futures::StreamExt;
    use tokio_util::compat::TokioAsyncReadCompatExt;

    let reader = BufReader::new(reader);
    let decoder = XzDecoder::new(reader).compat();
    let archive = Archive::new(decoder);

    tokio::fs::create_dir_all(target).await?;

    let mut entries = archive.entries()?;

    while let Some(entry) = entries.next().await {
        let mut entry = entry?;
        let path = entry.path()?;

        let path_str = match path.to_str() {
            Some(s) => s,
            None => continue,
        };

        let Some(safe_path) = enclosed_name(path_str) else {
            continue;
        };

        let full_path = target.join(safe_path);

        if entry.header().entry_type().is_dir() {
            tokio::fs::create_dir_all(&full_path).await?;
        } else {
            if let Some(parent) = full_path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            entry.unpack(&full_path).await?;
        }
    }

    Ok(())
}

pub async fn extract<R: AsyncRead + Unpin>(reader: R, path: &Path, target: &Path) -> Result<()> {
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    #[cfg(not(windows))]
    if filename.ends_with(".tar.xz") {
        return untar_xz(reader, target).await;
    }

    #[cfg(windows)]
    if filename.ends_with(".zip") {
        return unzip(reader, target).await;
    }

    bail!("Unsupported archive type: {}", path.display())
}
