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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = RequestConfig::default();
        assert_eq!(config.url, None);
        assert_eq!(config.method, Method::GET);
        assert_eq!(config.base_url, None);
        assert_eq!(config.timeout, Some(1000));
        assert_eq!(config.headers, None);
        assert_eq!(config.params, None);
        assert_eq!(config.data, None);
    }

    #[test]
    fn test_update_config() {
        let mut config = RequestConfig::default();
        config.url = Some("https://api.example.com".to_string());
        config.method = Method::POST;
        config.base_url = Some("https://api.example.com".to_string());
        config.timeout = Some(2000);
        config.headers = None;
        config.params = None;
        config.data = None;

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
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", "Bearer token".parse().unwrap());
        config.headers = Some(headers.clone());

        assert_eq!(config.headers, Some(headers));
    }
}
