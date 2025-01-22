use crate::RequestConfig;

pub mod echo_http;
pub mod echo_internal;
pub mod echo_errors;

pub struct Echo {
    pub config: RequestConfig,
    client: reqwest::Client,
}
