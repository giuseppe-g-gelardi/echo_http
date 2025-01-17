use crate::RequestConfig;

pub mod echo;
pub mod echo_internal;

pub struct Echo {
    pub config: RequestConfig,
    client: reqwest::Client,
}
