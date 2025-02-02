use crate::{Echo, EchoError, Response, ResponseUnknown};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use super::{HttpClient, RequestHandler};

#[async_trait]
impl HttpClient for Echo<'_> {
    async fn get<T>(&self, url: &str) -> Result<Response<T>, EchoError>
    where
        T: Serialize + DeserializeOwned + Send,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.get(&full_url);
        self.send_request(request, url, None::<()>).await
    }

    async fn post<T>(&self, url: &str, data: Option<T>) -> Result<Response<T>, EchoError>
    where
        T: Serialize + DeserializeOwned + Send,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.post(&full_url);
        self.send_request(request, url, data).await
    }

    async fn put<T>(&self, url: &str, data: Option<T>) -> Result<Response<T>, EchoError>
    where
        T: Serialize + DeserializeOwned + Send,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.put(&full_url);
        self.send_request(request, url, data).await
    }

    async fn delete(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
        let full_url = self.get_full_url(url);
        let request = self.client.delete(&full_url);
        self.send_request_unknown(request, url, None::<()>).await
    }

    async fn get_unknown(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
        let full_url = self.get_full_url(url);
        let request = self.client.get(&full_url);
        self.send_request_unknown(request, url, None::<()>).await
    }

    async fn post_no(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
        let full_url = self.get_full_url(url);
        let request = self.client.post(&full_url);
        self.send_request_unknown(request, url, None::<()>).await
    }
}

// impl<'a> Echo<'a> {
//     pub fn configure(config: Option<RequestConfig<'a>>) -> Self {
//         let config = config.unwrap_or_default();

//         Echo {
//             config,
//             client: reqwest::Client::new(),
//         }
//     }

//     /// get request for an unknown endpoint
//     /// ```rs
//     /// let mut config = RequestConfig::default();
//     /// config.base_url = Some("https://jsonplaceholder.typicode.com/".to_string());
//     ///
//     /// let echo = Echo::configure(Some(config));
//     ///
//     /// let response = echo.get_unknown("/users/1").await?;
//     /// ```
//     pub async fn get_unknown(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
//         let full_url = self.get_full_url(url);
//         let request = self.client.get(&full_url);
//         self.send_request_unknown(request, url, Nope).await
//     }

//     /// get request
//     /// ```rs
//     /// let echo = Echo::configure(None);
//     /// let res = echo.get::<T>("https://jsonplaceholder.typicode.com/").await?;
//     /// ```
//     pub async fn get<T>(&self, url: &str) -> Result<Response<T>, EchoError>
//     where
//         T: serde::Serialize + serde::de::DeserializeOwned,
//     {
//         let full_url = self.get_full_url(url);
//         let request = self.client.get(&full_url);
//         self.send_request(request, url, Nope).await
//     }

//     /// post request
//     /// ```rs
//     /// let echo = Echo::configure(...);
//     ///
//     /// let res = echo.post::<User>("/users", Some(new_user)).await?;
//     /// ```
//     pub async fn post<T>(&self, url: &str, data: Option<T>) -> Result<Response<T>, EchoError>
//     where
//         T: serde::Serialize + serde::de::DeserializeOwned,
//     {
//         let full_url = self.get_full_url(url);
//         let request = self.client.post(&full_url);
//         self.send_request(request, url, data).await
//     }

//     /// post request with no data
//     /// ```rs
//     /// let echo = Echo::configure(None);
//     /// let res = echo.post_no("https://jsonplaceholder.typicode.com/posts").await?;
//     ///
//     /// post_no is used when you want to send a post request with no data
//     /// ```
//     pub async fn post_no(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
//         let full_url = self.get_full_url(url);
//         let request = self.client.post(&full_url);
//         self.send_request_unknown::<serde_json::Value>(request, url, None)
//             .await
//     }

//     /// put request
//     /// ```rs
//     /// let echo = Echo::configure(None);
//     /// #[derive(Debug, Serialize, Deserialize)]
//     /// #[serde(rename_all = "camelCase")]
//     /// struct Post {
//     ///    user_id: u16,
//     ///    id: u16,
//     ///    title: String,
//     ///    body: String,
//     /// }
//     ///
//     /// let updated_post = Post {
//     ///    user_id: 1,
//     ///    id: 1,
//     ///    title: "post title".to_string(),
//     ///    body: "compelling post body".to_string(),
//     /// };
//     ///
//     /// let put = echo.put::<Post>("https://jsonplaceholder.typicode.com/posts/1", Some(updated_post)).await?;
//     /// ```
//     pub async fn put<T>(&self, url: &str, data: Option<T>) -> Result<Response<T>, EchoError>
//     where
//         T: serde::Serialize + serde::de::DeserializeOwned,
//     {
//         let full_url = self.get_full_url(url);
//         let request = self.client.put(&full_url);
//         self.send_request(request, url, data).await
//     }

//     /// delete request
//     /// ```rs
//     /// let echo = Echo::configure(None);
//     /// let deleted = echo.delete("https://jsonplaceholder.typicode.com/posts/1").await?;
//     /// ```
//     /// `response.data` should return an empty object.
//     pub async fn delete(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
//         let full_url = self.get_full_url(url);
//         let request = self.client.delete(&full_url);
//         self.send_request_unknown(request, url, Nope).await
//     }
// }
