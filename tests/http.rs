use echo_http::{Echo, Headers, RequestConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    user_id: u16,
    id: u16,
    title: String,
    body: String,
}

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

#[tokio::test]
async fn test_get_unknown() {
    let mut config = RequestConfig::default();
    config.base_url = Some("https://jsonplaceholder.typicode.com/".to_string());

    let echo = Echo::configure(Some(config));

    let response = echo.get_unknown("/users/1").await.unwrap();

    assert_eq!(response.status_text, "OK")
}

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

// test post with no data
// uses serde_json::Value && None
#[tokio::test]
async fn test_post_no_data() {
    let echo = Echo::configure(None);

    let response = echo
        .post::<serde_json::Value>("https://jsonplaceholder.typicode.com/posts/", None)
        .await
        .unwrap();
    assert_eq!(response.status, 201);
    assert_eq!(response.status_text, "Created")
}

// simplified version of test_post_no_data
#[tokio::test]
async fn test_post_no_data_2() {
    let mut config = RequestConfig::default();
    let mut headers = Headers::new();
    config.base_url = Some("https://jsonplaceholder.typicode.com/".to_string());

    headers.insert("Content-Type: application/json");
    config.headers = Some(headers.clone());

    let echo = Echo::configure(Some(config.clone()));

    let response = echo.post_no("posts").await.unwrap();

    assert_eq!(response.status, 201);
    assert_eq!(response.status_text, "Created")
}

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

#[tokio::test]
async fn test_delete() {
    let echo = Echo::configure(None);

    let deleted = echo
        .delete("https://jsonplaceholder.typicode.com/posts/1")
        .await
        .unwrap();

    assert_eq!(deleted.status, 200);
    assert_eq!(deleted.status_text, "OK");
}
