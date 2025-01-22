use super::echo_errors::EchoError;
use crate::{Echo, Nope, RequestConfig, Response, ResponseUnknown};

impl Echo {
    /// Create an Echo instance with the `configure()` method.
    /// configure takes an Option;
    /// ```rs
    /// Option<Config> or None
    /// 
    /// Configure the Echo instance with an optional base URL
    ///
    /// let echo_config = Config {
    ///     base_url: Some("https://jsonplaceholder.typicode.com/".to_string()),
    ///     timeout: None,
    ///     headers: None,
    /// };
    /// let config_withurl = Echo::configure(Some(echo_config));
    ///
    /// or
    ///
    /// passing None allows you to send a request to a full url
    /// let echo = Echo::configure(None);
    ///
    /// let res = echo.get("https://jsonplaceholder.typicode.com/users/1")
    /// ```
    ///
    pub fn configure(config: Option<RequestConfig>) -> Self {
        let config = config.unwrap_or_default();

        Echo {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// get request for an unknown endpoint
    /// ```rs
    /// let mut config = RequestConfig::default();
    /// config.base_url = Some("https://jsonplaceholder.typicode.com/".to_string());
    ///
    /// let echo = Echo::configure(Some(config));
    ///
    /// let response = echo.get_unknown("/users/1").await.unwrap();
    /// ```
    pub async fn get_unknown(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
        let full_url = self.get_full_url(url);
        let request = self.client.get(&full_url);
        self.send_request_unknown(request, url, Nope).await
    }

    /// get request
    /// ```rs
    /// let echo = Echo::configure(None);
    /// let res = echo.get<Type>("https://jsonplaceholder.typicode.com/").await?;
    /// ```
    pub async fn get<T>(&self, url: &str) -> Result<Response<T>, EchoError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.get(&full_url);
        self.send_request(request, url, Nope).await
    }

    /// post request
    /// # example:
    /// ```rs
    /// let echo = Echo::configure(config{...});
    ///
    /// let res = echo.post("/users", Nope).await?;
    /// ```
    pub async fn post<T>(&self, url: &str, data: Option<T>) -> Result<Response<T>, EchoError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.post(&full_url);
        self.send_request(request, url, data).await
    }

    /// put request
    /// # example:
    /// ```rs
    /// let echo = Echo::configure(None);
    /// #[derive(Debug, Serialize, Deserialize)]
    /// #[serde(rename_all = "camelCase")]
    /// struct Post {
    ///    user_id: u16,
    ///    id: u16,
    ///    title: String,
    ///    body: String,
    /// }
    ///
    /// let updated_post = Post {
    ///    user_id: 1,
    ///    id: 1,
    ///    title: "post title".to_string(),
    ///    body: "compelling post body".to_string(),
    /// };
    ///
    /// let put = echo.put::<Post>("https://jsonplaceholder.typicode.com/posts/1", Some(updated_post)).await?;
    /// ```
    pub async fn put<T>(&self, url: &str, data: Option<T>) -> Result<Response<T>, EchoError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.put(&full_url);
        self.send_request(request, url, data).await
    }

    /// delete request
    /// # example
    /// ```rs
    /// let echo = Echo::configure(None);
    /// let deleted = echo.delete("https://jsonplaceholder.typicode.com/posts/1").await?;
    /// ```
    /// `response.data` should return an empty object. it will look like this: `Object {}`
    /// but it will be equal to `serde_json::json!({})`
    pub async fn delete<T>(&self, url: &str) -> Result<Response<T>, EchoError>
    where
        T: serde::de::DeserializeOwned,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.delete(&full_url);
        self.send_request(request, url, Nope).await
    }
}

#[cfg(test)]
mod tests {
    use crate::Nope;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Post {
        user_id: u16,
        id: u16,
        title: String,
        body: String,
    }

    #[ignore = "dont want to ddos jsonplaceholder"]
    #[tokio::test]
    async fn test_get() {
        let echo = Echo::configure(None);
        let posts = echo
            .get::<Post>("https://jsonplaceholder.typicode.com/posts/1")
            .await
            .unwrap();

        let post = posts.data;

        assert_eq!(post.id, 1);
        assert_eq!(post.user_id, 1);
    }

    #[ignore = "dont want to ddos jsonplaceholder"]
    #[tokio::test]
    async fn test_get_unknown() {
        let mut config = RequestConfig::default();
        config.base_url = Some("https://jsonplaceholder.typicode.com/".to_string());

        let echo = Echo::configure(Some(config));

        let response = echo.get_unknown("/users/1").await.unwrap();

        assert_eq!(response.status_text, "OK")
    }

    #[ignore = "dont want to ddos jsonplaceholder"]
    #[tokio::test]
    async fn test_post() {
        let echo = Echo::configure(None);
        let new_post = Post {
            user_id: 1,
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
        };

        let response = echo
            .post::<Post>(
                "https://jsonplaceholder.typicode.com/posts/",
                Some(new_post),
            )
            .await
            .unwrap();
        assert_eq!(response.status, 201);
        assert_eq!(response.status_text, "Created")
    }

    #[ignore = "dont want to ddos jsonplaceholder"]
    #[tokio::test]
    async fn test_post_no_data() {
        let echo = Echo::configure(None);
        let response = echo
            .post("https://jsonplaceholder.typicode.com/users/", Nope)
            .await
            .unwrap();

        assert_eq!(response.status, 201)
    }

    #[ignore = "dont want to ddos jsonplaceholder"]
    #[tokio::test]
    async fn test_put() {
        let echo = Echo::configure(None);

        let new_post = Post {
            user_id: 1,
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
        };

        let put = echo
            .put::<Post>(
                "https://jsonplaceholder.typicode.com/posts/1",
                Some(new_post),
            )
            .await
            .unwrap();

        assert_eq!(put.data.title, "title".to_string());
        assert_eq!(put.data.body, "body".to_string());
    }

    #[ignore = "dont want to ddos jsonplaceholder"]
    #[tokio::test]
    async fn test_delete() {
        let echo = Echo::configure(None);

        let deleted = echo
            .delete::<Post>("https://jsonplaceholder.typicode.com/posts/1")
            .await
            .unwrap();

        assert_eq!(deleted.status, 200);
        assert_eq!(deleted.status_text, "OK");
    }
}
