pub mod request_config;

use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use crate::headers::Headers;

/// Request Configuration
#[derive(Debug, Clone)]
pub struct RequestConfig {
    /// `url` is the server URL that will be used for the request
    pub url: Option<String>,

    /// `method` is the request method to be used when making the request
    pub method: Method, // 'GET' is the default

    /// `baseURL` will be prepended to `url` unless `url` is absolute.
    /// It can be convenient to set `baseURL` for an instance of echo_http to pass relative URLs
    /// to methods of that instance.
    pub base_url: Option<String>,

    /// `headers` are custom headers to be sent
    pub headers: Option<Headers>,

    /// `params` are the URL parameters to be sent with the request
    /// Must be a plain object or a URLSearchParams object
    pub params: Option<HashMap<String, String>>,

    /// `data` is the data to be sent as the request body
    /// Only applicable for request methods 'PUT', 'POST', 'DELETE , and 'PATCH' (soon &trade;)
    /// When no `transformRequest` (soon &trade;) is set, must be of one of the following types:
    /// - string, plain object, ArrayBuffer, ArrayBufferView, URLSearchParams
    /// - Browser only: FormData, File, Blob
    /// - Node only: Stream, Buffer, FormData (form-data package)
    pub data: Option<Value>,

    /// `timeout` specifies the number of milliseconds before the request times out.
    /// If the request takes longer than `timeout`, the request will be aborted.
    /// the default value is `0` (no timeout)
    pub timeout: Option<u64>,

    /// `responseType` indicates the type of data that the server will respond with
    /// options are: 'arraybuffer', 'document', 'json', 'text', 'stream'
    pub response_type: ResponseType, // default is JSON
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseType {
    ArrayBuffer,
    Document,
    Json,
    Text,
    Stream,
}

impl std::str::FromStr for ResponseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "arraybuffer" => Ok(ResponseType::ArrayBuffer),
            "document" => Ok(ResponseType::Document),
            "json" => Ok(ResponseType::Json),
            "text" => Ok(ResponseType::Text),
            "stream" => Ok(ResponseType::Stream),
            _ => Err(format!("Invalid response type: {}", s)),
        }
    }
}
