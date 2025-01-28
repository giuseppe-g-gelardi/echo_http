pub mod request_config;

use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use crate::headers::Headers;

/// Request Configuration
#[derive(Debug, Clone)]
pub struct RequestConfig<'a> {
    /// `url` is the server URL that will be used for the request
    pub url: Option<String>,

    /// `method` is the request method to be used when making the request
    pub method: Method, // 'GET' is the default

    /// `baseURL` will be prepended to `url` unless `url` is absolute.
    /// It can be convenient to set `baseURL` for an instance of echo_http to pass relative URLs
    /// to methods of that instance.
    pub base_url: Option<String>,

    /// `headers` are custom headers to be sent
    pub headers: Option<Headers<'a>>,

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
    //
    // (soon &trade;)
    // `xsrfCookieName` is the name of the cookie to use as a value for xsrf token
    //xsrf_cookie_name: Option<String>, // xsrfCookieName: 'XSRF-TOKEN', // default

    // `xsrfHeaderName` is the name of the http header that carries the xsrf token value
    //xsrf_header_name: Option<String>, // xsrfHeaderName: 'X-XSRF-TOKEN', // default

    // `undefined` (default) - set XSRF header only for the same origin requests
    //with_xsrf_token: Option<bool>, // withXSRFToken: boolean | undefined | ((config: Internalecho_httpRequestConfig) => boolean | undefined),
}
