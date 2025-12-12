use std::path::PathBuf;
use tokio::process::Command;

pub struct WebTorrentCommand {
    inner: Command,
}

impl WebTorrentCommand {
    pub(crate) fn new(node_path: &PathBuf, webtorrent_path: &PathBuf) -> Self {
        let mut command = Command::new(node_path);
        command.arg(webtorrent_path);

        Self { inner: command }
    }

    pub fn download(mut self, torrent_id: &str) -> Self {
        self.inner.arg("download");
        self.inner.arg(torrent_id);
        self
    }

    pub fn download_meta(mut self, torrent_id: &str) -> Self {
        self.inner.arg("downloadmeta");
        self.inner.arg(torrent_id);
        self
    }

    pub fn seed(mut self, input: &str) -> Self {
        self.inner.arg("seed");
        self.inner.arg(input);
        self
    }

    pub fn info(mut self, torrent_id: &str) -> Self {
        self.inner.arg("info");
        self.inner.arg(torrent_id);
        self
    }

    pub fn out(mut self, path: &str) -> Self {
        self.inner.arg("-o");
        self.inner.arg(path);
        self
    }

    pub fn select(mut self, index: usize) -> Self {
        self.inner.arg("-s");
        self.inner.arg(index.to_string());
        self
    }

    pub fn interactive_select(mut self) -> Self {
        self.inner.arg("-i");
        self
    }

    pub fn subtitles(mut self, path: &str) -> Self {
        self.inner.arg("-t");
        self.inner.arg(path);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.inner.arg("-p");
        self.inner.arg(port.to_string());
        self
    }

    pub fn blocklist(mut self, path: &str) -> Self {
        self.inner.arg("-b");
        self.inner.arg(path);
        self
    }

    pub fn announce(mut self, url: &str) -> Self {
        self.inner.arg("-a");
        self.inner.arg(url);
        self
    }

    pub fn quiet(mut self) -> Self {
        self.inner.arg("-q");
        self
    }

    pub fn download_limit(mut self, limit: u32) -> Self {
        self.inner.arg("-d");
        self.inner.arg(limit.to_string());
        self
    }

    pub fn upload_limit(mut self, limit: u32) -> Self {
        self.inner.arg("-u");
        self.inner.arg(limit.to_string());
        self
    }

    pub fn verbose(mut self) -> Self {
        self.inner.arg("--verbose");
        self
    }

    pub fn torrent_port(mut self, port: u16) -> Self {
        self.inner.arg("--torrent-port");
        self.inner.arg(port.to_string());
        self
    }

    pub fn dht_port(mut self, port: u16) -> Self {
        self.inner.arg("--dht-port");
        self.inner.arg(port.to_string());
        self
    }

    pub fn keep_seeding(mut self) -> Self {
        self.inner.arg("--keep-seeding");
        self
    }

    pub fn on_done(mut self, script: &str) -> Self {
        self.inner.arg("--on-done");
        self.inner.arg(script);
        self
    }

    pub fn on_exit(mut self, script: &str) -> Self {
        self.inner.arg("--on-exit");
        self.inner.arg(script);
        self
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.inner.arg(arg);
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        self.inner.args(args);
        self
    }

    pub async fn spawn(mut self) -> std::io::Result<tokio::process::Child> {
        self.inner.spawn()
    }

    pub async fn output(mut self) -> std::io::Result<std::process::Output> {
        self.inner.output().await
    }

    pub async fn status(mut self) -> std::io::Result<std::process::ExitStatus> {
        self.inner.status().await
    }

    pub fn inner_mut(&mut self) -> &mut Command {
        &mut self.inner
    }
}
