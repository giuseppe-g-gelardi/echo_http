use crate::RequestConfig;

pub mod http;
pub mod echo_internal;
pub mod echo_unknown;
pub mod http_client;
pub mod request_handler;

pub use http_client::HttpClient;
pub use request_handler::RequestHandler;

pub struct Echo<'a> {
    pub config: RequestConfig<'a>,
    client: reqwest::Client,
}

impl <'a> Echo<'a> {
    /// Create an Echo instance with the `configure()` method.
    /// configure takes an Option;
    /// ```rs
    /// Option<Config> or None
    ///
    /// Configure the Echo instance a few different ways
    ///
    /// let mut config = RequestConfig::default();
    /// config.base_url = Some("https://jsonplaceholder.typicode.com/posts/1".to_string());
    ///
    /// let echo = Echo::configure(Some(config));
    ///
    /// or
    ///
    /// passing None allows you to send a request to a full url
    /// let echo = Echo::configure(None);
    ///
    /// let res = echo.get("https://jsonplaceholder.typicode.com/users/1")
    /// ```
    pub fn configure(config: Option<RequestConfig<'a>>) -> Self {
        let config = config.unwrap_or_default();
        let client = reqwest::Client::new();
        Echo { config, client }
    }
}
