use axum::http::HeaderMap;
use m3u8_rs::Playlist;
use url::Url;

use crate::utils::create_proxy_url;

/// Checks if the given URL and headers indicate that the content is a M3U8 file.
pub fn is_m3u8_content(url: &Url, headers: &HeaderMap) -> bool {
    let has_m3u8_extension = url.path().ends_with(".m3u8");

    let has_m3u8_content_type = headers
        .get("content-type")
        .and_then(|header_value| header_value.to_str().ok())
        .is_some_and(|content_type| {
            [
                "application/vnd.apple.mpegurl",
                "application/x-mpegURL",
                "audio/mpegurl",
            ]
            .iter()
            .any(|mime_type| content_type.contains(mime_type))
        });

    has_m3u8_extension || has_m3u8_content_type
}

/// Rewrites the given M3U8 to use the proxy URLs.
pub fn rewrite_m3u8(proxy_url: &Url, base_uri: &Url, bytes: &[u8]) -> Vec<u8> {
    // TOOD: Handle this possible unwrap error
    let (_i, playlist) = m3u8_rs::parse_playlist(bytes).unwrap();
    let mut new_bytes = Vec::new();
    match playlist {
        Playlist::MasterPlaylist(mut pl) => {
            pl.alternatives.iter_mut().for_each(|alt| {
                if let Some(uri) = &alt.uri {
                    alt.uri = Some(create_proxy_url(proxy_url, base_uri, uri));
                }
            });
            pl.variants.iter_mut().for_each(|variant| {
                variant.uri = create_proxy_url(proxy_url, base_uri, &variant.uri);
            });
            pl.write_to(&mut new_bytes).unwrap();
        }
        Playlist::MediaPlaylist(mut pl) => {
            pl.segments.iter_mut().for_each(|segment| {
                if let Some(map) = segment.map.as_mut() {
                    map.uri = create_proxy_url(proxy_url, base_uri, &map.uri);
                }
                if let Some(uri) = segment.key.as_mut().and_then(|key| key.uri.as_mut()) {
                    *uri = create_proxy_url(proxy_url, base_uri, uri);
                }
                segment.uri = create_proxy_url(proxy_url, base_uri, &segment.uri);
            });
            pl.write_to(&mut new_bytes).unwrap();
        }
    }
    new_bytes
}
