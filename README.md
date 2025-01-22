# echo_http

`echo_http` is a type-safe HTTP client library built with Rust, based on Axios, for the typescript devs that just can't let go

## Features

- Type-safe HTTP requests and responses
- Async support with `reqwest`
- Generic response deserialization

## Example Usage

```rs
use echo_http::{Echo, Response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let echo = Echo::configure(None);

    let posts = echo
        .get<Vec<Post>>("https://jsonplaceholder.typicode.com/posts")
        .await?
        .data // you can chain .data to get the Response.data directly. cool, right?

    for post in posts {
        println!("Post ID: {}, Title: {}", post.id, post.title);
    }
}
