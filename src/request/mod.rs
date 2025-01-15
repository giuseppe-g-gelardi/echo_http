pub mod request;

use reqwest::header::HeaderMap;
use std::collections::HashMap;
use serde_json::Value;
use crate::method::Method;

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: Method, 
    pub base_url: Option<String>,
    pub headers: Option<HeaderMap>,
    pub params: Option<HashMap<String, String>>,
    pub data: Option<Value>,
    pub timeout: Option<u64>
}

