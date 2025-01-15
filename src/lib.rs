pub struct Axios {
    pub base_url: Option<String>,
}

impl Axios {
    // pub fn configure(url: Option<String>) -> Axios {
    //     // let parsed_url = if url.ends_with("/") {
    //     //     url.trim_end_matches("/").to_string()
    //     // } else {
    //     //     url
    //     // };
    //
    //     let parsed_url = url.map(|u| {
    //         if u.ends_with("/") {
    //             u.trim_end_matches("/").to_string()
    //         } else {
    //             u
    //         }
    //     });
    //
    //     Axios {
    //         // base_url: Some(parsed_url),
    //         base_url: parsed_url,
    //     }
    // }
    pub fn configure<U>(url: U) -> Self
    where
        U: Into<Option<String>>,
    {
        let parsed_url = url.into().map(|u| {
            if u.ends_with("/") {
                u.trim_end_matches("/").to_string()
            } else {
                u
            }
        });

        Axios {
            base_url: parsed_url,
        }
    }

    pub async fn get(&self, url: String) -> String {
        if let Some(base_url) = &self.base_url {
            // Trim leading slash from the endpoint if necessary
            let parsed_endpoint = if url.starts_with("/") {
                url.trim_start_matches("/").to_string()
            } else {
                url
            };

            // Combine base URL with the endpoint
            format!("{}/{}", base_url, parsed_endpoint)
        } else {
            // If base_url is None, use the provided URL as-is
            url
        }
        // do the request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_url() {
        let result = Axios::configure("google.com".to_string());
        assert_eq!(result.base_url, "google.com".to_string().into());
    }

    #[test]
    fn test_url_parsed() {
        let result = Axios::configure("http://github.com/".to_string());
        assert_eq!(result.base_url, Some("http://github.com".to_string()));
    }

    #[tokio::test]
    async fn test_get_output() {
        let config = Axios::configure("http://github.com/".to_string());
        let result = config.get("users".to_string()).await;
        let expected = "http://github.com/users".to_string();
        assert_eq!(result, expected)
    }

    #[tokio::test]
    async fn test_no_base_url() {
        let config = Axios::configure(None);
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
