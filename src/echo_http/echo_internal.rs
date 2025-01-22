use crate::{Echo, Response};
use serde_json::Value;

use super::echo_errors::EchoError;

impl Echo {
    /// method to parse leading and or trailing slashes from the url
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

    async fn parse_response(
        &self,
        response: reqwest::Response,
        url: &str,
    ) -> Result<Response, EchoError> {
        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("")
            .to_string();
        let headers = response.headers().clone();

        let data: Value = response.json().await.unwrap_or_else(|_| Value::Null);

        Ok(Response {
            data,
            status,
            status_text,
            headers,
            config: self.config.clone(),
            request: self.get_full_url(url),
        })
    }

    pub(crate) async fn send_request<T>(
        &self,
        mut request: reqwest::RequestBuilder,
        url: &str,
        body: Option<T>,
    ) -> Result<Response, EchoError>
    where
        T: serde::Serialize,
    {
        request = self.apply_headers(request);
        request = self.apply_timeout(request);
        request = self.apply_body(request, body);
        request = self.apply_params(request);

        let response = request.send().await?;
        self.parse_response(response, url).await


        // let status = response.status().as_u16();
        // let status_text = response
        //     .status()
        //     .canonical_reason()
        //     .unwrap_or("")
        //     .to_string();
        // let headers = response.headers().clone();
        //
        // let data: Value = response.json().await.unwrap_or_else(|_| Value::Null);

        // Ok(Response {
        //     data,
        //     status,
        //     status_text,
        //     headers,
        //     config: self.config.clone(),
        //     request: self.get_full_url(url),
        // })
    }
}
