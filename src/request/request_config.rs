use super::{RequestConfig, ResponseType};
use reqwest::Method;

impl Default for RequestConfig {
    fn default() -> Self {
        RequestConfig {
            url: None,
            method: Method::GET,
            base_url: None,
            timeout: None,
            headers: None,
            params: None,
            data: None,
            response_type: ResponseType::Json,
        }
    }
}
