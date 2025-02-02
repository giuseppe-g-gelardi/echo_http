use crate::{EchoError, Response, ResponseUnknown};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait HttpClient {
    async fn get<T>(&self, url: &str) -> Result<Response<T>, EchoError>
    where
        T: Serialize + DeserializeOwned + Send;

    async fn post<T>(&self, url: &str, data: Option<T>) -> Result<Response<T>, EchoError>
    where
        T: Serialize + DeserializeOwned + Send;

    async fn put<T>(&self, url: &str, data: Option<T>) -> Result<Response<T>, EchoError>
    where
        T: Serialize + DeserializeOwned + Send;

    async fn delete(&self, url: &str) -> Result<ResponseUnknown, EchoError>;

    async fn get_unknown(&self, url: &str) -> Result<ResponseUnknown, EchoError>;

    async fn post_no(&self, url: &str) -> Result<ResponseUnknown, EchoError>;
}
