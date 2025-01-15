pub struct Echo {
    pub base_url: Option<String>,
}

impl Echo {
    /// Configure the Echo instance with an optional base URL
    pub fn configure<U>(url: U) -> Self
    where
        U: Into<Option<String>>,
    {
        let parsed_url = url.into().map(|u| Self::parse_url(&u));

        Echo {
            base_url: parsed_url,
        }
    }

    /// get request
    pub async fn get(&self, url: String) -> String {
        if let Some(base_url) = &self.base_url {
            let parsed_endpoint = Self::parse_url(&url);
            format!("{}/{}", base_url, parsed_endpoint)
        } else {
            url
        }
        // do the request
    }

    fn parse_url(url: &str) -> String {
        let url = url.trim_start_matches("/").trim_end_matches("/");

        url.to_string()
    }
}
struct Headers(String, String); // ::new() // new takes 2 strings and parses correctly
struct Respo<T, C, R> {
    data: T,
    status: i32,
    status_text: String,
    headers: Headers, // sigh..
    config: C,        // Echo::config
    request: R,       // duh..
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_url() {
        let result = Echo::configure("google.com".to_string());
        assert_eq!(result.base_url, "google.com".to_string().into());
    }

    #[test]
    fn test_url_parsed() {
        let result = Echo::configure("http://github.com/".to_string());
        assert_eq!(result.base_url, Some("http://github.com".to_string()));
    }

    #[tokio::test]
    async fn test_get_output() {
        let config = Echo::configure("http://github.com/".to_string());
        let result = config.get("users".to_string()).await;
        let expected = "http://github.com/users".to_string();
        assert_eq!(result, expected)
    }

    #[tokio::test]
    async fn test_no_base_url() {
        let config = Echo::configure(None);
        let res = config.get("http://google.com".to_string()).await;
        let expected = "http://google.com".to_string();
        assert_eq!(res, expected)
    }
}

// pub use Request {
//
//     pub enum Method {
//         GET,
//         POST,
//         PUT,
//         DELETE,
//     }
//
//     pub struct Request {
//         method: Method,
//         headers: Option<(String, String)>,
//         body: String, // JSON, XML w/ post or put
//         url: String,  // is there a URL/Path type?
//     }
//
//     pub struct Response {
//         status_code: i32, // 200 OK, 404 Not Found, etc...
//         headers: String,  // Content-Type: "application/json"
//         body: String,     // JSON or HTML
//     }
