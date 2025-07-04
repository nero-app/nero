use std::collections::HashMap;

use url::Url;

pub fn extract_target_url(url: &Url) -> Option<String> {
    url.query_pairs()
        .find(|(key, _)| key == "target")
        .map(|(_, value)| value.into_owned())
}

pub fn extract_headers(url: &Url) -> HashMap<String, String> {
    url.query_pairs()
        .find(|(key, _)| key == "headers")
        .map(|(_, value)| {
            value
                .split('|')
                .filter_map(|header_pair| {
                    header_pair
                        .split_once(':')
                        .map(|(name, val)| (name.trim().to_owned(), val.trim().to_owned()))
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn create_proxy_url(proxy_url: &Url, base_uri: &Url, uri: &str) -> String {
    let target_url = match Url::parse(uri) {
        Ok(absolute_url) => absolute_url,
        Err(_) => base_uri.join(uri).unwrap(),
    };
    Url::parse_with_params(proxy_url.as_str(), &[("target", target_url.as_str())])
        .unwrap()
        .to_string()
}
