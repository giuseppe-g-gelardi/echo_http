use super::{RequestConfig, ResponseType};
use reqwest::Method;

impl<'a> Default for RequestConfig<'a> {
    fn default() -> Self {
        RequestConfig {
            url: None,
            method: Method::GET,
            base_url: None,
            timeout: Some(0),
            headers: None,
            params: None,
            data: None,
            response_type: ResponseType::Json,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::headers::Headers;

    use super::*;

    #[test]
    fn test_default() {
        let config = RequestConfig::default();
        assert_eq!(config.url, None);
        assert_eq!(config.method, Method::GET);
        assert_eq!(config.base_url, None);
        assert_eq!(config.timeout, Some(0));
        assert_eq!(config.headers, None);
        assert_eq!(config.params, None);
        assert_eq!(config.data, None);
        assert_eq!(config.response_type, ResponseType::Json);
    }

    #[test]
    fn test_update_config() {
        let mut config = RequestConfig::default();
        config.url = Some("https://api.example.com".to_string());
        config.method = Method::POST;
        config.base_url = Some("https://api.example.com".to_string());
        config.timeout = Some(2000);
        ..RequestConfig::default();
        // config.headers = None;
        // config.params = None;
        // config.data = None;

        assert_eq!(config.url, Some("https://api.example.com".to_string()));
        assert_eq!(config.method, Method::POST);
        assert_eq!(config.base_url, Some("https://api.example.com".to_string()));
        assert_eq!(config.timeout, Some(2000));
        assert_eq!(config.headers, None);
        assert_eq!(config.params, None);
        assert_eq!(config.data, None);
    }

    #[test]
    fn test_update_headers() {
        let mut config = RequestConfig::default();
        let mut headers = Headers::new();
        headers.insert("Content-Type: application/json");
        headers.insert("Authorization: Bearer token");
        config.headers = Some(headers.clone());

        assert_eq!(config.headers, Some(headers));
    }

    #[test]
    fn test_default_request_config() {
        let config: RequestConfig = RequestConfig::default();
        assert_eq!(config.response_type, ResponseType::Json);
        assert_eq!(config.timeout, Some(0));
    }

    #[test]
    fn test_parse_response_type() {
        assert_eq!("json".parse::<ResponseType>().unwrap(), ResponseType::Json);
        assert_eq!("text".parse::<ResponseType>().unwrap(), ResponseType::Text);
        assert!("invalid".parse::<ResponseType>().is_err());
    }

    #[test]
    fn test_request_config_with_lifetime() {
        let config = RequestConfig {
            timeout: Some(1000),
            response_type: ResponseType::Text,
            ..RequestConfig::default()
        };

        assert_eq!(config.timeout, Some(1000));
        assert_eq!(config.response_type, ResponseType::Text);
    }
}
