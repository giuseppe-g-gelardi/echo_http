use reqwest::header::HeaderMap;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: Option<String>,
    pub timeout: Option<u64>,
    pub headers: Option<Vec<(String, String)>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base_url: None,
            timeout: None,
            headers: None,
        }
    }
}

pub struct Echo {
    pub config: Config,
    client: reqwest::Client,
}

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
        let full_url = if let Some(base_url) = &self.config.base_url {
            let parsed_endpoint = Self::parse_url(url);
            format!("{}/{}", base_url, parsed_endpoint)
        } else {
            url.to_string()
        };

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

        // let body = response.text().await?;
        // Ok(body)
        Ok(Response {
            data,
            status,
            status_text,
            headers,
            config: self.config.clone(),
            request: full_url,
        })
    }

    fn parse_url(url: &str) -> String {
        let url = url.trim_start_matches("/").trim_end_matches("/");

        url.to_string()
    }
}
#[derive(Debug)]
pub struct Response {
    pub data: Value,
    pub status: u16,
    pub status_text: String,
    pub headers: HeaderMap, // sigh..
    pub config: Config,     // Echo::config
    pub request: String,    // duh..
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lol() {
        assert_eq!(2, 2)
    }

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
}
