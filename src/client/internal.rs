use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};

use crate::{request::ResponseType, Echo, EchoError, Response, ResponseUnknown};

#[derive(Debug)]
pub enum ParsedResponse<'a, T> {
    Response(Response<'a, T>),
    ResponseUnknown(ResponseUnknown<'a>),
}

impl Echo<'_> {
    async fn parse<T>(
        &self,
        response: reqwest::Response,
        url: &str,
        is_unknown_response: bool,
    ) -> Result<ParsedResponse<'_, T>, EchoError>
    where
        T: DeserializeOwned + Send,
    {
        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("")
            .to_string();
        let headers = response.headers().clone();

        match is_unknown_response {
            true => {
                let data: serde_json::Value =
                    response.json().await.unwrap_or(serde_json::Value::Null);
                Ok(ParsedResponse::ResponseUnknown(ResponseUnknown {
                    inner: Response {
                        data,
                        status,
                        status_text,
                        headers,
                        config: self.config.clone(),
                        request: self.get_full_url(url),
                    },
                }))
            }
            false => {
                let data = self.handle_response_type::<T>(response).await?;
                Ok(ParsedResponse::Response(Response {
                    data,
                    status,
                    status_text,
                    headers,
                    config: self.config.clone(),
                    request: self.get_full_url(url),
                }))
            }
        }
    }

    pub async fn send<T, U>(
        &self,
        mut request: RequestBuilder,
        url: &str,
        body: Option<T>,
        is_unknown_response: bool,
    ) -> Result<ParsedResponse<'_, U>, EchoError>
    where
        T: Serialize + Send,
        U: DeserializeOwned + Send,
    {
        request = self.apply_headers(request);
        request = self.apply_timeout(request);
        request = self.apply_body(request, body);
        request = self.apply_params(request);

        let response = request.send().await?;
        self.parse(response, url, is_unknown_response).await
    }
}

impl Echo<'_> {
    async fn parse_response<T>(
        &self,
        response: reqwest::Response,
        url: &str,
    ) -> Result<Response<T>, EchoError>
    where
        T: DeserializeOwned + Send,
    {
        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("")
            .to_string();
        let headers = response.headers().clone();

        let data = self.handle_response_type::<T>(response).await?;

        Ok(Response {
            data,
            status,
            status_text,
            headers,
            config: self.config.clone(),
            request: self.get_full_url(url),
        })
    }

    pub async fn parse_response_unknown(
        &self,
        response: reqwest::Response,
        url: &str,
    ) -> Result<ResponseUnknown, EchoError> {
        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("")
            .to_string();
        let headers = response.headers().clone();

        let data: serde_json::Value = response.json().await.unwrap_or(serde_json::Value::Null);

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

    pub async fn send_request<T, U>(
        &self,
        mut request: RequestBuilder,
        url: &str,
        body: Option<T>,
    ) -> Result<Response<U>, EchoError>
    where
        T: Serialize + Send,
        U: DeserializeOwned + Send,
    {
        request = self.apply_headers(request);
        request = self.apply_timeout(request);
        request = self.apply_body(request, body);
        request = self.apply_params(request);

        let response = request.send().await?;
        self.parse_response(response, url).await
    }

    pub async fn send_request_unknown<T>(
        &self,
        mut request: RequestBuilder,
        url: &str,
        body: Option<T>,
    ) -> Result<ResponseUnknown, EchoError>
    where
        T: Serialize + Send,
    {
        request = self.apply_headers(request);
        request = self.apply_timeout(request);
        request = self.apply_body(request, body);
        request = self.apply_params(request);

        let response = request.send().await?;
        self.parse_response_unknown(response, url).await
    }
}

impl<'a> Echo<'a> {
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

    pub(crate) fn apply_headers(
        &self,
        mut request: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        if let Some(headers) = &self.config.headers {
            let header_map: reqwest::header::HeaderMap = headers.clone().into();
            request = request.headers(header_map);
        }
        request
    }

    pub(crate) fn apply_timeout(
        &self,
        mut request: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        if let Some(timeout) = self.config.timeout {
            request = request.timeout(std::time::Duration::from_secs(timeout))
        }
        request
    }

    pub(crate) fn apply_params(
        &self,
        mut request: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        if let Some(params) = &self.config.params {
            request = request.query(params);
        }
        request
    }

    pub(crate) fn apply_body<T>(
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

    async fn handle_response_type<T>(&self, response: reqwest::Response) -> Result<T, EchoError>
    where
        T: serde::de::DeserializeOwned,
    {
        match self.config.response_type {
            ResponseType::Json => response.json::<T>().await.map_err(EchoError::from),
            ResponseType::Text => {
                let text = response.text().await.map_err(EchoError::from)?;
                serde_json::from_str(&text).map_err(EchoError::from)
            }
            ResponseType::ArrayBuffer => {
                let bytes = response.bytes().await.map_err(EchoError::from)?;
                serde_json::from_slice(&bytes).map_err(EchoError::from)
            }
            _ => Err(EchoError::UnsupportedResponseType(
                self.config.response_type,
            )),
        }
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
