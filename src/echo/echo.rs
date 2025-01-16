use crate::{Config, Echo, Response};
use serde_json::Value;

impl Echo {
    // /// Configure the Echo instance with an optional base URL
    pub fn configure(config: Option<Config>) -> Self {
        let config = config.unwrap_or_default();

        Echo {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// get request
    pub async fn get(&self, url: &str) -> Result<Response, reqwest::Error> {
        let full_url = self.get_full_url(url);

        let mut request = self.client.get(&full_url);

        if let Some(headers) = &self.config.headers {
            for (key, value) in headers {
                request = request.header(key, value)
            }
        }

        if let Some(timeout) = self.config.timeout {
            request = request.timeout(std::time::Duration::from_secs(timeout));
        }

        let response = request.send().await?;

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
            request: full_url,
        })
    }

    fn get_full_url(&self, url: &str) -> String {
        if let Some(base_url) = &self.config.base_url {
            let parsed_endpoint = Self::parse_url(url);
            format!("{}/{}", base_url, parsed_endpoint)
        } else {
            url.to_string()
        }
    }

    /// post request
    pub async fn post<T>(&self, url: &str, data: Option<T>) -> Result<Response, reqwest::Error>
    where
        T: serde::Serialize,
    {
        let full_url = self.get_full_url(url);

        let mut request = self.client.post(&full_url);

        if let Some(headers) = &self.config.headers {
            for (key, value) in headers {
                request = request.header(key, value)
            }
        }

        if let Some(timeout) = self.config.timeout {
            request = request.timeout(std::time::Duration::from_secs(timeout));
        }

        if let Some(body) = data {
            request = request.json(&body);
        }

        let response = request.send().await?;

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
            request: full_url,
        })
    }

    /// method to parse leading and or trailing slashes from the url
    fn parse_url(url: &str) -> String {
        let url = url.trim_start_matches("/").trim_end_matches("/");

        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Nope;

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
            .await.unwrap();

        assert_eq!(response.status, 201)
    }
}
