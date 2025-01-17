pub mod request_config;

use crate::method::Method;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub url: Option<String>,
    pub method: Method, // 'GET' is the default
    pub base_url: Option<String>,
    pub headers: Option<HeaderMap>,
    pub params: Option<HashMap<String, String>>,
    pub data: Option<Value>,
    pub timeout: Option<u64>,
}
