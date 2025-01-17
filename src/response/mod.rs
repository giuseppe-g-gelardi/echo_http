pub mod response;

// use crate::Config;

use reqwest::header::HeaderMap;
use serde_json::Value;

use crate::RequestConfig;

#[derive(Debug)]
pub struct Response {
    pub data: Value,
    pub status: u16,
    pub status_text: String,
    pub headers: HeaderMap,
    pub config: RequestConfig,
    pub request: String,
}
