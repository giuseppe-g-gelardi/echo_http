pub mod response;

use std::ops::Deref;

use crate::RequestConfig;

#[derive(Debug)]
pub struct Response<'a, T> {
    pub data: T,
    pub status: u16,
    pub status_text: String,
    pub headers: reqwest::header::HeaderMap,
    pub config: RequestConfig<'a>,
    pub request: String,
}

#[derive(Debug)]
pub struct ResponseUnknown<'a> {
    pub inner: Response<'a, serde_json::Value>,
}

impl<'a> Deref for ResponseUnknown<'a> {
    type Target = Response<'a, serde_json::Value>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
