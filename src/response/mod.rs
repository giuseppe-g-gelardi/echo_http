pub mod response;

use crate::Config;

use reqwest::header::HeaderMap;
use serde_json::Value;

#[derive(Debug)]
pub struct Response {
    pub data: Value,
    pub status: u16,
    pub status_text: String,
    pub headers: HeaderMap, // sigh..
    pub config: Config,     // Echo::config
    pub request: String,    // duh..
}
