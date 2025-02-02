use crate::RequestConfig;

pub mod http;
pub mod echo_internal;
pub mod echo_unknown;
pub mod http_client;

pub use http_client::HttpClient;

pub struct Echo<'a> {
    pub config: RequestConfig<'a>,
    client: reqwest::Client,
}

impl <'a> Echo<'a> {
    pub fn configure(config: Option<RequestConfig<'a>>) -> Self {
        let config = config.unwrap_or_default();
        let client = reqwest::Client::new();
        Echo { config, client }
    }
}
