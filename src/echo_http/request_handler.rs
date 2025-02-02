use crate::{EchoError, Response, ResponseUnknown};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use reqwest::RequestBuilder;

#[async_trait]
pub trait RequestHandler {
    async fn parse_response<T>(
        &self,
        response: reqwest::Response,
        url: &str,
    ) -> Result<Response<T>, EchoError>
    where
        T: DeserializeOwned + Send;

    async fn parse_response_unknown(
        &self,
        response: reqwest::Response,
        url: &str,
    ) -> Result<ResponseUnknown, EchoError>;

    async fn send_request<T, U>(
        &self,
        request: RequestBuilder,
        url: &str,
        body: Option<T>,
    ) -> Result<Response<U>, EchoError>
    where
        T: Serialize + Send,
        U: DeserializeOwned + Send;

    async fn send_request_unknown<T>(
        &self,
        request: RequestBuilder,
        url: &str,
        body: Option<T>,
    ) -> Result<ResponseUnknown, EchoError>
    where
        T: Serialize + Send;
}
