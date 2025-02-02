use echo_http::request::ResponseType;
use echo_http::{Headers, RequestConfig, Response, ResponseUnknown};
use serde_json::json;

#[test]
fn test_response_creation() {
    let mut headers = Headers::new();
    headers.insert("Content-Type: application/json");

    let config = RequestConfig {
        base_url: Some("https://api.example.com".to_string()),
        timeout: Some(30),
        headers: Some(headers.clone()),
        params: None,
        response_type: ResponseType::Json,
        ..RequestConfig::default()
    };

    let response = Response {
        data: json!({ "message": "success" }),
        status: 200,
        status_text: "OK".to_string(),
        headers: headers.into(),
        config,
        request: "https://api.example.com/test".to_string(),
    };

    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert_eq!(response.request, "https://api.example.com/test");
    assert_eq!(response.data["message"], "success");
}

#[test]
fn test_response_unknown() {
    let mut headers = Headers::new();
    headers.insert("Content-Type: application/json");

    let config = RequestConfig {
        base_url: Some("https://api.example.com".to_string()),
        timeout: Some(30),
        headers: Some(headers.clone()),
        params: None,
        response_type: ResponseType::Json,
        ..RequestConfig::default()
    };

    let response = Response {
        data: json!({ "id": 123, "name": "Test" }),
        status: 201,
        status_text: "Created".to_string(),
        headers: headers.into(),
        config,
        request: "https://api.example.com/users".to_string(),
    };

    let response_unknown = ResponseUnknown { inner: response };

    assert_eq!(response_unknown.status, 201);
    assert_eq!(response_unknown.status_text, "Created");
    assert_eq!(response_unknown.request, "https://api.example.com/users");
    assert_eq!(response_unknown.data["id"], 123);
}
