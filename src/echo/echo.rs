use crate::{Config, Echo, Nope, Response};
// use serde_json::Value;

impl Echo {
    /// Create an Echo instance with the `configure()` method.
    /// configure takes an Option;
    /// ```rs
    /// Option<Config>
    /// ```
    /// or
    /// ```rs
    /// None
    /// ```
    ///
    /// Configure the Echo instance with an optional base URL
    /// ```rs
    /// let echo_config = Config {
    ///     base_url: Some("https://jsonplaceholder.typicode.com/".to_string()),
    ///     timeout: None,
    ///     headers: None,
    /// };
    /// ```
    ///
    /// ```rs
    /// let config_withurl = Echo::configure(Some(echo_config));```
    /// or
    /// ```rs
    /// let echo = Echo::configure(None)```
    ///
    /// passing None allows you to send a request to a full url
    /// example:
    /// ```rs
    /// let res = echo.get("https://jsonplaceholder.typicode.com/users/1")
    /// ```
    ///
    pub fn configure(config: Option<Config>) -> Self {
        let config = config.unwrap_or_default();

        Echo {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// get request
    /// ```rs
    /// let echo = Echo::configure(None);
    /// let res = echo.get("https://jsonplaceholder.typicode.com/").await?;
    /// ```
    pub async fn get(&self, url: &str) -> Result<Response, reqwest::Error> {
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
    pub async fn post<T>(&self, url: &str, data: Option<T>) -> Result<Response, reqwest::Error>
    where
        T: serde::Serialize,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.post(&full_url);
        self.send_request(request, url, data).await
    }

    /// put request
    /// # example:
    /// ```rs
    /// let echo = Echo::configure(None);
    /// #[derive(Debug, Serialize)]
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
    pub async fn put<T>(&self, url: &str, data: Option<T>) -> Result<Response, reqwest::Error>
    where
        T: serde::Serialize,
    {
        let full_url = self.get_full_url(url);
        let request = self.client.put(&full_url);
        self.send_request(request, url, data).await
    }
}

#[cfg(test)]
mod tests {
    use crate::Nope;
    use serde::Serialize;

    use super::*;

    #[tokio::test]
    async fn test_get() {
        let config = Config {
            base_url: Some("https://jsonplaceholder.typicode.com/".to_string()),
            timeout: None,
            headers: None,
        };

        let echo = Echo::configure(Some(config));

        let response = echo.get("/users/1").await.unwrap();

        assert_eq!(response.status_text, "OK")
    }

    #[tokio::test]
    async fn test_post_no_data() {
        let echo = Echo::configure(None);
        let response = echo
            .post("https://jsonplaceholder.typicode.com/users/", Nope)
            .await
            .unwrap();

        assert_eq!(response.status, 201)
    }

    #[tokio::test]
    async fn test_put() {
        let echo = Echo::configure(None);
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Post {
            user_id: u16,
            id: u16,
            title: String,
            body: String,
        }

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

        assert_eq!(put.data.get("title"), Some(&serde_json::json!("title")));
    }
}