use std::io::Cursor;

use m3u8_rs::Playlist;
use tauri::{
    Runtime,
    plugin::{Builder as PluginBuilder, TauriPlugin},
};
use tiny_http::{Header, Response, StatusCode};
use url::Url;

// TODO: Improve error handling
fn handle_request(request: tiny_http::Request) {
    let url = request.url();
    if let Some(target_url) = extract_target_url(url) {
        match proxy_request(&target_url, &request) {
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

// TODO: Extract headers from the url

fn extract_target_url(url: &str) -> Option<String> {
    url.find('?').and_then(|query_start| {
        url[query_start + 1..].split('&').find_map(|param| {
            param
                .split_once('=')
                .and_then(|(key, value)| {
                    matches!(key, "target").then(|| urlencoding::decode(value).ok())
                })
                .flatten()
                .map(|s| s.into_owned())
        })
    })
}

fn proxy_request(
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
        true => rewrite_m3u8(url.join("./")?, response.as_bytes()),
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

/// Checks if the given URL and headers indicate that the content is a M3U8 file.
fn is_m3u8_content(url: &Url, headers: &[Header]) -> bool {
    let has_m3u8_extension = url
        .path_segments()
        .and_then(|segments| segments.last())
        .is_some_and(|segment| segment.ends_with(".m3u8"));

    let has_m3u8_content_type = headers
        .iter()
        .find(|header| header.field.equiv("content-type"))
        .is_some_and(|header| {
            [
                "application/vnd.apple.mpegurl",
                "application/x-mpegURL",
                "audio/mpegurl",
            ]
            .iter()
            .any(|mime_type| header.value.as_str().contains(mime_type))
        });

    has_m3u8_extension || has_m3u8_content_type
}

fn create_proxy_url(base_uri: &Url, uri: &str) -> String {
    let target_url = match Url::parse(uri) {
        Ok(absolute_url) => absolute_url,
        Err(_) => base_uri.join(uri).unwrap(),
    };
    Url::parse_with_params("http://localhost:8080/", &[("target", target_url.as_str())])
        .unwrap()
        .to_string()
}

/// Rewrites the given M3U8 to use the proxy URLs.
fn rewrite_m3u8(base_uri: Url, bytes: &[u8]) -> Vec<u8> {
    // TOOD: Handle this possible unwrap error
    let (_i, playlist) = m3u8_rs::parse_playlist(bytes).unwrap();
    let mut new_bytes = Vec::new();
    match playlist {
        Playlist::MasterPlaylist(mut pl) => {
            pl.alternatives.iter_mut().for_each(|alt| {
                if let Some(uri) = &alt.uri {
                    alt.uri = Some(create_proxy_url(&base_uri, uri));
                }
            });
            pl.variants.iter_mut().for_each(|variant| {
                variant.uri = create_proxy_url(&base_uri, &variant.uri);
            });
            pl.write_to(&mut new_bytes).unwrap();
        }
        Playlist::MediaPlaylist(mut pl) => {
            pl.segments.iter_mut().for_each(|segment| {
                if let Some(map) = segment.map.as_mut() {
                    map.uri = create_proxy_url(&base_uri, &map.uri);
                }
                if let Some(uri) = segment.key.as_mut().and_then(|key| key.uri.as_mut()) {
                    *uri = create_proxy_url(&base_uri, uri);
                }
                segment.uri = create_proxy_url(&base_uri, &segment.uri);
            });
            pl.write_to(&mut new_bytes).unwrap();
        }
    }
    new_bytes
}

// TODO: Add configuration for a custom host, port, threads for the tiny_http server
// and timeout for the http requests?
pub struct Builder;

impl Builder {
    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        PluginBuilder::new("tauri-plugin-video-proxy")
            .setup(move |_app, _api| {
                std::thread::spawn(move || {
                    let server = tiny_http::Server::http("localhost:8080").unwrap();
                    for req in server.incoming_requests() {
                        handle_request(req);
                    }
                });
                Ok(())
            })
            .build()
    }
}
