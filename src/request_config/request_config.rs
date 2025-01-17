use crate::method::Method;

use super::RequestConfig;

impl Default for RequestConfig {
    fn default() -> Self {
        RequestConfig {
            url: None,
            method: Method::Get,
            base_url: None,
            timeout: None,
            headers: None,
            params: None,
            data: None,
        }
    }
}
