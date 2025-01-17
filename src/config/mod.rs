use reqwest::header::HeaderMap;

pub mod config;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: Option<String>,
    pub timeout: Option<u64>,
    pub headers: Option<HeaderMap>,
}
