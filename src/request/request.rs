use super::Request;
use crate::method::Method;

impl Request {
    /// Creates a new `Request` with required fields
    pub fn new(url: String, method: Method) -> Self {
        Self {
            url,
            method,
            base_url: None,
            headers: None,
            params: None,
            data: None,
            timeout: None,
        }
    }
}
