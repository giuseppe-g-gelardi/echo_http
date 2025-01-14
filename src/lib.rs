pub struct Axios {
    pub base_url: Option<String>,
}

impl Axios {
    pub fn configure(url: String) -> Axios {
        let parsed_url = if url.ends_with("/") {
            url.trim_end_matches("/").to_string()
        } else {
            url
        };

        Axios {
            base_url: Some(parsed_url),
        }
    }

    pub fn get(&self, endpoint: String) -> String {
        let base_url = self.base_url.as_ref().expect("Base URL is not configured");

        let parsed_endpoint = if endpoint.starts_with("/") {
            endpoint.trim_start_matches("/").to_string()
        } else {
            endpoint
        };

        format!("{}/{}", base_url, parsed_endpoint).to_string()
    }
}

// enum Method {
//     GET,
//     POST,
//     PUT,
//     DELETE,
// }
//
// struct Request {
//     method: Method,
//     headers: Option<(String, String)>,
//     body: String, // JSON, XML w/ post or put
//     url: String,  // is there a URL/Path type?
// }
//
// struct Response {
//     status_code: i32, // 200 OK, 404 Not Found, etc...
//     headers: String,  // Content-Type: "application/json"
//     body: String,     // JSON or HTML
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_url() {
        let result = Axios::configure("google.com".to_string());
        assert_eq!(result.base_url, Some("google.com".to_string()));
    }

    #[test]
    fn test_url_parsed() {
        let result = Axios::configure("http://github.com/".to_string());
        assert_eq!(result.base_url, Some("http://github.com".to_string()));
    }

    #[test]
    fn test_get_output() {
        let config = Axios::configure("http://github.com/".to_string());
        let result = config.get("users".to_string());
        let expected = "http://github.com/users".to_string();
        assert_eq!(result, expected)
    }
}
