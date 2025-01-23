use crate::response::ResponseUnknown;
use crate::{Echo, Response};

use super::echo_errors::EchoError;

impl Echo {
    fn parse_url(url: &str) -> String {
        let url = url.trim_start_matches("/").trim_end_matches("/");

        url.to_string()
    }

    pub(crate) fn get_full_url(&self, url: &str) -> String {
        if let Some(base_url) = &self.config.base_url {
            let parsed_endpoint = Self::parse_url(url);
            format!("{}/{}", base_url, parsed_endpoint)
        } else {
            url.to_string()
        }
    }

    fn apply_headers(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(headers) = &self.config.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        request
    }

    fn apply_timeout(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(timeout) = self.config.timeout {
            request = request.timeout(std::time::Duration::from_secs(timeout))
        }
        request
    }

    fn apply_params(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(params) = &self.config.params {
            request = request.query(params);
            // todo! implement parsing for params
        }
        request
    }

    fn apply_body<T>(
        &self,
        mut request: reqwest::RequestBuilder,
        body: Option<T>,
    ) -> reqwest::RequestBuilder
    where
        T: serde::Serialize,
    {
        if let Some(body) = body {
            request = request.json(&body);
        }
        request
    }

    async fn parse_response<T>(
        &self,
        response: reqwest::Response,
        url: &str,
    ) -> Result<Response<T>, EchoError>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("")
            .to_string();
        let headers = response.headers().clone();

        let data = if response.status().is_success() {
            response.json::<T>().await? // Deserialize directly to T
        } else {
            panic!("Unexpected response body or error for URL: {}", url)
        };

        Ok(Response {
            data,
            status,
            status_text,
            headers,
            config: self.config.clone(),
            request: self.get_full_url(url),
        })
    }

    pub(crate) async fn send_request<T, U>(
        &self,
        mut request: reqwest::RequestBuilder,
        url: &str,
        body: Option<T>,
    ) -> Result<Response<U>, EchoError>
    where
        T: serde::Serialize,
        U: serde::de::DeserializeOwned,
    {
        request = self.apply_headers(request);
        request = self.apply_timeout(request);
        request = self.apply_body(request, body);
        request = self.apply_params(request);

        let response = request.send().await?;
        self.parse_response(response, url).await
    }

    // quick and dirty implementation for now
    pub(crate) async fn send_request_unknown<T>(
        &self,
        mut request: reqwest::RequestBuilder,
        url: &str,
        body: Option<T>,
    ) -> Result<ResponseUnknown, EchoError>
    where
        T: serde::Serialize,
        // U: serde::de::DeserializeOwned,
    {
        request = self.apply_headers(request);
        request = self.apply_timeout(request);
        request = self.apply_body(request, body);
        request = self.apply_params(request);

        let response = request.send().await?;

        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("")
            .to_string();
        let headers = response.headers().clone();

        let data: serde_json::Value = response
            .json()
            .await
            .unwrap_or_else(|_| serde_json::Value::Null);

        Ok(ResponseUnknown {
            inner: Response {
                data,
                status,
                status_text,
                headers,
                config: self.config.clone(),
                request: self.get_full_url(url),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RequestConfig;

    #[test]
    fn test_parse_url() {
        assert_eq!(Echo::parse_url("/endpoint/"), "endpoint");
        assert_eq!(Echo::parse_url("endpoint/"), "endpoint");
        assert_eq!(Echo::parse_url("/endpoint"), "endpoint");
        assert_eq!(Echo::parse_url("endpoint"), "endpoint");
    }

    #[test]
    fn test_get_full_url_with_base_url() {
        let config = RequestConfig {
            base_url: Some("https://api.example.com".to_string()),
            ..Default::default()
        };
        let echo = Echo::configure(Some(config));

        assert_eq!(
            echo.get_full_url("/endpoint"),
            "https://api.example.com/endpoint"
        );
        assert_eq!(
            echo.get_full_url("endpoint/"),
            "https://api.example.com/endpoint"
        );
    }

    #[test]
    fn test_get_full_url_without_base_url() {
        let echo = Echo::configure(None);
        assert_eq!(
            echo.get_full_url("https://api.example.com/endpoint"),
            "https://api.example.com/endpoint"
        );
    }
}
