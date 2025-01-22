use crate::RequestConfig;

pub mod echo_http;
pub mod echo_internal;

pub struct Echo {
    pub config: RequestConfig,
    client: reqwest::Client,
}
