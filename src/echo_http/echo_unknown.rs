use crate::{Echo, EchoError, Response, ResponseUnknown};

impl<'a> Echo<'a> {
    async fn parse_response_unknown(
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

    pub(crate) async fn send_request_unknown<T>(
        &self,
        mut request: reqwest::RequestBuilder,
        url: &str,
        body: Option<T>,
    ) -> Result<ResponseUnknown, EchoError>
    where
        T: serde::Serialize,
    {
        request = self.apply_headers(request);
        request = self.apply_timeout(request);
        request = self.apply_body(request, body);
        request = self.apply_params(request);

        let response = request.send().await?;
        self.parse_response_unknown(response, url).await
    }
}
