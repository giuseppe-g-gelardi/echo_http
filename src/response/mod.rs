pub mod response;

// use reqwest::header::HeaderMap;
// use serde_json::Value;

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
