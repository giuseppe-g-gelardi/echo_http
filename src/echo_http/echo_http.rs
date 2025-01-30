use crate::{Echo, EchoError, Nope, RequestConfig, Response, ResponseUnknown};

impl<'a> Echo<'a> {
    /// Create an Echo instance with the `configure()` method.
    /// configure takes an Option;
    /// ```rs
    /// Option<Config> or None
    ///
    /// Configure the Echo instance a few different ways
    ///
    /// let mut config = RequestConfig::default();
    /// config.base_url = Some("https://jsonplaceholder.typicode.com/posts/1".to_string());
    ///
    /// let echo = Echo::configure(Some(config));
    ///
    /// or
    ///
    /// passing None allows you to send a request to a full url
    /// let echo = Echo::configure(None);
    ///
    /// let res = echo.get("https://jsonplaceholder.typicode.com/users/1")
    /// ```
    ///
    pub fn configure(config: Option<RequestConfig<'a>>) -> Self {
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
    /// let response = echo.get_unknown("/users/1").await?;
    /// ```
    pub async fn get_unknown(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
        let full_url = self.get_full_url(url);
        let request = self.client.get(&full_url);
        self.send_request_unknown(request, url, Nope).await
    }

    /// get request
    /// ```rs
    /// let echo = Echo::configure(None);
    /// let res = echo.get::<T>("https://jsonplaceholder.typicode.com/").await?;
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
    /// ```rs
    /// let echo = Echo::configure(...);
    ///
    /// let res = echo.post::<User>("/users", Some(new_user)).await?;
    ///
    /// - note: in order to send a post request with no data, you must pass `None` and the type
    /// argument must be `serde_json::Value`
    ///
    /// let res = echo.post::<serde_json::Value>("https://jsonplaceholder.typicode.com/posts/", None)
    /// its janky, but it works. will be fixed in the future.
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
    /// ```rs
    /// let echo = Echo::configure(None);
    /// let deleted = echo.delete("https://jsonplaceholder.typicode.com/posts/1").await?;
    /// ```
    /// `response.data` should return an empty object.
    pub async fn delete(&self, url: &str) -> Result<ResponseUnknown, EchoError> {
        let full_url = self.get_full_url(url);
        let request = self.client.delete(&full_url);
        self.send_request_unknown(request, url, Nope).await
    }
}
