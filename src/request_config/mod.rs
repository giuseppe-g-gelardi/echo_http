pub mod request_config;

use crate::method::Method;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;

/// Request Configuration
/// `url` is the server URL that will be used for the request
/// `method` is the request method to be used when making the request
/// `base_url` will be prepended to `url` unless `url` is absolute. It can be convenient to set `baseURL` for an instance of axios to pass relative URLs to methods of that instance.
/// 'headers' are custom headers to be sent
/// `params` are URL parameters to be sent with the request. #TODO! must be plain object or
/// URLSearchParams object
/// `data` is the data to be sent as the request body.
/// `timeout` specifies the number of milliseconds before the request times out. If the request takes longer than `timeout`, the request will be aborted.
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
