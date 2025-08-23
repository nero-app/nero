use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{
    Deserialize, Deserializer,
    de::{self, DeserializeOwned},
};

fn decode_base64<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|encoded| STANDARD.decode(&encoded).map_err(de::Error::custom))
        .transpose()
}

pub fn deserialize_base64_opt<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    decode_base64(deserializer)?
        .map(|bytes| serde_json::from_slice(&bytes).map_err(de::Error::custom))
        .transpose()
}

pub fn deserialize_base64_plain_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    decode_base64(deserializer)?
        .map(|bytes| String::from_utf8(bytes).map_err(de::Error::custom))
        .transpose()
}

#[cfg(test)]
mod tests {
    use crate::proxy::TargetHttpResource;

    use base64::{Engine, engine::general_purpose::STANDARD};
    use url::Url;

    #[test]
    fn deserialize_minimal_without_headers_or_body() {
        let query = "method=GET&url=https%3A%2F%2Fexample.com";
        let resource: TargetHttpResource = serde_qs::from_str(query).unwrap();

        assert_eq!(resource.method, "GET");
        assert_eq!(resource.url.as_str(), "https://example.com/");
        assert!(resource.headers.is_none());
        assert!(resource.body.is_none());
    }

    #[test]
    fn deserialize_with_headers_and_plain_text_body() {
        let headers = [
            ("Content-Type".to_owned(), "application/json".to_owned()),
            ("Authorization".to_owned(), "Bearer token".to_owned()),
        ];
        let headers_json = serde_json::to_string(&headers).unwrap();
        let headers_b64 = STANDARD.encode(headers_json);

        let body_plain = "Hello, this is plain text!";
        let body_b64 = STANDARD.encode(body_plain);

        let target_url = "https://api.example.com/data?x=1&y=2";
        let query = format!(
            "method=POST&url={}&headers={}&body={}",
            urlencoding::encode(target_url),
            urlencoding::encode(&headers_b64),
            urlencoding::encode(&body_b64)
        );

        let resource: TargetHttpResource = serde_qs::from_str(&query).unwrap();

        assert_eq!(resource.method, "POST");
        assert_eq!(resource.url.as_str(), target_url);
        assert_eq!(resource.headers.unwrap(), headers);
        assert_eq!(resource.body.unwrap(), body_plain);
    }

    #[test]
    fn fail_if_headers_is_invalid_base64() {
        let query = "method=GET&url=https%3A%2F%2Fexample.com&headers=###INVALID###";
        let result: Result<TargetHttpResource, _> = serde_qs::from_str(query);
        assert!(result.is_err());
    }

    #[test]
    fn fail_if_headers_base64_is_not_valid_json() {
        let bad_json = STANDARD.encode("not json at all");
        let query = format!(
            "method=GET&url=https%3A%2F%2Fexample.com&headers={}",
            urlencoding::encode(&bad_json)
        );
        let result: Result<TargetHttpResource, _> = serde_qs::from_str(&query);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_body_with_json_content() {
        let json_body = r#"{"user":"John Doe"}"#;
        let body_b64 = STANDARD.encode(json_body);

        let query = format!(
            "method=POST&url={}&body={}",
            urlencoding::encode("https://example.com"),
            urlencoding::encode(&body_b64)
        );

        let resource: TargetHttpResource = serde_qs::from_str(&query).unwrap();

        assert_eq!(resource.body.unwrap(), json_body);
    }

    #[test]
    fn deserialize_body_with_xml_content() {
        let xml_body = r#"<note><to>User</to><from>Server</from></note>"#;
        let body_b64 = STANDARD.encode(xml_body);

        let query = format!(
            "method=POST&url={}&body={}",
            urlencoding::encode("https://example.com"),
            urlencoding::encode(&body_b64)
        );

        let resource: TargetHttpResource = serde_qs::from_str(&query).unwrap();

        assert_eq!(resource.body.unwrap(), xml_body);
    }
}
