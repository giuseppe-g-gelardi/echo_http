use super::Headers;

use reqwest::header::HeaderMap;
use std::collections::HashMap;

/// A struct for managing headers.
impl<'a> Headers<'a> {
    /// Creates a new `Headers` instance.
    pub fn new() -> Self {
        Headers {
            headers: HashMap::new(),
        }
    }

    /// Inserts a single header in the format "key: value".
    /// ```rs
    /// use echo_http::Headers;
    ///
    /// let mut headers = Headers::new();
    /// headers.insert("Content-Type: application/json");
    pub fn insert(&mut self, header: &'a str) {
        if let Some((key, value)) = header.split_once(':') {
            self.headers.insert(key.trim(), value.trim());
        } else {
            panic!("Header must be in the format 'key: value'");
        }
    }

    /// Inserts multiple headers from a vector of strings.
    /// ```rs
    /// use echo_http::Headers;
    ///
    /// let mut headers = Headers::new();
    ///
    /// headers.insert_many(vec![
    ///    "Content-Type: application/json",
    ///    "Authorization: Bearer token",
    ///    "X-Api-Key: secret",
    /// ]);
    pub fn insert_many(&mut self, headers: Vec<&'a str>) {
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
impl Into<HeaderMap> for Headers<'_> {
    fn into(self) -> HeaderMap {
        self.to_header_map()
    }
}

