use echo_http::Headers;
use reqwest::header::HeaderMap;

#[test]
fn test_insert_single_header() {
    let mut headers = Headers::new();
    headers.insert("Content-Type: application/json");

    assert_eq!(
        headers.headers.get("Content-Type"),
        Some(&"application/json".to_string())
    );
}

#[test]
fn test_insert_many_headers() {
    let mut headers = Headers::new();
    headers.insert_many(vec![
        "Content-Type: application/json",
        "Authorization: Bearer token",
    ]);

    assert_eq!(
        headers.headers.get("Content-Type"),
        Some(&"application/json".to_string())
    );
    assert_eq!(
        headers.headers.get("Authorization"),
        Some(&"Bearer token".to_string())
    );
}

#[test]
fn test_into_header_map() {
    let mut headers = Headers::new();
    headers.insert("Content-Type: application/json");
    headers.insert("Authorization: Bearer token");

    let header_map: HeaderMap = headers.into();
    assert_eq!(header_map["Content-Type"], "application/json");
    assert_eq!(header_map["Authorization"], "Bearer token");
}
