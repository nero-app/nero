mod m3u8;
mod utils;

use std::io::Cursor;

use tauri::{
    Runtime,
    plugin::{Builder as PluginBuilder, TauriPlugin},
};
use tiny_http::{Response, StatusCode};
use url::Url;
use urlencoding::decode;

use crate::{
    m3u8::{is_m3u8_content, rewrite_m3u8},
    utils::extract_target_url,
};

pub struct ProxyServer {
    host: String,
    port: u16,
}

// TODO: Improve error handling
impl ProxyServer {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    fn run(self) {
        let server_addr = format!("{}:{}", self.host, self.port);
        let server = tiny_http::Server::http(&server_addr).unwrap();
        println!("Proxy server running on: {}", server_addr);

        for req in server.incoming_requests() {
            self.handle_request(req);
        }
    }

    fn handle_request(&self, request: tiny_http::Request) {
        let full_url = format!("http://{}:{}{}", self.host, self.port, request.url());
        let decoded_url = decode(&full_url).unwrap();
        let url = Url::parse(&decoded_url).unwrap();

        if let Some(target_url) = extract_target_url(&url) {
            match self.proxy_request(&target_url, &request) {
                Ok(response) => {
                    let _ = request.respond(response);
                }
                Err(error) => {
                    let error_response =
                        tiny_http::Response::from_string(error.to_string()).with_status_code(500);
                    let _ = request.respond(error_response);
                }
            }
        } else {
            let error_response =
                tiny_http::Response::from_string("Missing target parameter").with_status_code(400);
            let _ = request.respond(error_response);
        }
    }

    fn proxy_request(
        &self,
        target_url: &str,
        req: &tiny_http::Request,
    ) -> Result<tiny_http::Response<Cursor<Vec<u8>>>, Box<dyn std::error::Error>> {
        let url = Url::parse(target_url).unwrap();

        let mut request = minreq::get(target_url);
        for header in req.headers() {
            // TODO: Remove this when compressed responses are supported
            if header.field.equiv("accept-encoding") {
                continue;
            }
            // minreq sets the "host" header automatically
            if header.field.equiv("host") {
                continue;
            }
            request = request.with_header(header.field.to_string(), header.value.as_str());
        }
        let response = request.with_timeout(30).send()?;

        // TODO: Add flate2 to handle compressed responses

        let mut headers = Vec::with_capacity(response.headers.len());
        for (key, value) in &response.headers {
            // Other headers such as connection, trailer, transfer-encoding and upgrade,
            // are ignored when creating the proxy response with tiny_http::Response::new.
            if key == "content-length" {
                continue;
            }
            headers.push(tiny_http::Header::from_bytes(key.as_str(), value.as_bytes()).unwrap());
        }

        let status_code = response.status_code as u16;

        let has_range_header = req.headers().iter().any(|h| h.field.equiv("range"));
        let should_rewrite_m3u8 = is_m3u8_content(&url, &headers) && !has_range_header;
        let body_bytes = match should_rewrite_m3u8 {
            true => {
                let proxy_url = Url::parse(&format!("http://{}:{}", self.host, self.port)).unwrap();
                rewrite_m3u8(&proxy_url, &url.join("./")?, response.as_bytes())
            }
            // For other resources, read the bytes and send the response.
            // This assumes that <video> when making a request to a resource such as an mp4,
            // will make the request with "range" headers, because otherwise,
            // the server will take a long time to download all the bytes of the video...
            false => response.into_bytes(),
        };
        let body_len = body_bytes.len();

        let proxy_response = Response::new(
            StatusCode(status_code),
            headers,
            Cursor::new(body_bytes),
            Some(body_len),
            None,
        );
        Ok(proxy_response)
    }
}

pub struct Builder {
    host: String,
    port: u16,
}

impl Builder {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        let proxy_server = ProxyServer::new(self.host, self.port);
        PluginBuilder::new("tauri-plugin-video-proxy")
            .setup(move |_app, _api| {
                std::thread::spawn(move || {
                    proxy_server.run();
                });
                Ok(())
            })
            .build()
    }
}
