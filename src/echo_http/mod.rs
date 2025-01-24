use crate::RequestConfig;

pub mod echo_http;
pub mod echo_internal;
pub mod echo_errors;

pub struct Echo<'a> {
    pub config: RequestConfig<'a>,
    client: reqwest::Client,
}
