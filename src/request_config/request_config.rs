use super::RequestConfig;
use reqwest::Method;

impl Default for RequestConfig {
    fn default() -> Self {
        RequestConfig {
            url: None,
            method: Method::GET,
            base_url: None,
            timeout: Some(1000),
            headers: None,
            params: None,
            data: None,
        }
    }
}
