pub mod response;

// use reqwest::header::HeaderMap;
// use serde_json::Value;

use std::ops::Deref;

use crate::RequestConfig;

#[derive(Debug)]
pub struct Response<T> {
    pub data: T,
    pub status: u16,
    pub status_text: String,
    pub headers: reqwest::header::HeaderMap,
    pub config: RequestConfig,
    pub request: String,
}

#[derive(Debug)]
pub struct ResponseUnknown {
    pub inner: Response<serde_json::Value>,
}

impl Deref for ResponseUnknown {
    type Target = Response<serde_json::Value>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
