use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    /// Creates a new `Headers` instance.
    pub fn new() -> Self {
        Headers {
            headers: HashMap::new(),
        }
    }

    /// Inserts a single header in the format "key: value".
    pub fn insert(&mut self, header: &str) {
        if let Some((key, value)) = header.split_once(':') {
            self.headers
                .insert(key.trim().to_string(), value.trim().to_string());
        } else {
            panic!("Header must be in the format 'key: value'");
        }
    }

    /// Inserts multiple headers from a vector of strings.
    pub fn insert_many(&mut self, headers: Vec<&str>) {
        for header in headers {
            self.insert(header);
        }
    }

    /// Converts the internal representation to `reqwest::header::HeaderMap`.
    fn to_header_map(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        for (key, value) in &self.headers {
            header_map.insert(
                reqwest::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
                value.parse().unwrap(),
            );
        }
        header_map
    }
}

/// Automatically convert `Headers` into `reqwest::header::HeaderMap`.
impl Into<HeaderMap> for Headers {
    fn into(self) -> HeaderMap {
        self.to_header_map()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_single_header() {
        let mut headers = Headers::new();
        headers.insert("Content-Type: application/json");

        assert_eq!(
            headers.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_insert_many_headers() {
        let mut headers = Headers::new();
        headers.insert_many(vec![
            "Content-Type: application/json",
            "Authorization: Bearer token",
        ]);

        assert_eq!(
            headers.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(
            headers.headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
    }

    #[test]
    fn test_into_header_map() {
        let mut headers = Headers::new();
        headers.insert("Content-Type: application/json");
        headers.insert("Authorization: Bearer token");

        let header_map: HeaderMap = headers.into();
        assert_eq!(header_map["Content-Type"], "application/json");
        assert_eq!(header_map["Authorization"], "Bearer token");
    }
}
