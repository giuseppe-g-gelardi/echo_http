# echo_http

`echo_http` is a type-safe HTTP client library built with Rust, based on Axios, for the typescript devs that just can't let go

## Features

- Type-safe HTTP requests and responses
- Async support with `reqwest`
- Generic response deserialization

## Getting started

### Requirements

- An **async runtime** (e.g., [tokio](https://tokio.rs/)).
- `serde` for data serialization/deserialization.
- Optional: `reqwest` if you need advanced features not provided by `echo_http` directly.

### Installation
```toml
[dependencies]
echo_http = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
# reqwest = { version = "0.12", features = ["json"] }
```

## Example Usage

### In a hurry?
just use `echo_http::echo;`
* This is a static default instance. No need to configure anything, just bring into scope and go!
* works with all supported methods: get, post, put and delete
```rs
use echo_http::echo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = echo.get<Vec<Post>>("https://jsonplaceholder.typicode.com/posts").await?;
    println!("{}", res.data);
    Ok(())
}
```

### Managing Headers with the `Headers` API
* The `Headers` struct offers a user-friendly way to manage request headers without working directly with `reqwest::HeaderMap`.

#### Inserting headers:
```rs
use echo_http::{Headers, Echo, RequestConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = Headers::new();
    headers.insert("Content-Type: application/json");
    headers.insert("Authorization: Bearer token");

    let mut echo_config = RequestConfig::default();
    echo_config.headers = Some(headers);

    {........}
}
```

#### Inserting multiple headers:
```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = Headers::new();
    headers.insert_many(vec![
        "Content-Type: application/json",
        "Authorization: Bearer token",
    ]);

    let mut echo_config = RequestConfig::default();
    echo_config.headers = Some(headers);

    {........}
}

```

### Have no idea what data type youre expecting?
* if the response type is unclear, use `get_unknown` to retreive a `serde_json::Value`
* *Only supports GET requests at this time. 
```rs
use echo_http::echo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // hmmm this api has no documentation, i wonder what the data type is...
    let idk = echo.get_unknown("https://mysterious.internal.api/").await?;
    println!("🙈 {:#?}", idk)

    Ok(())
}
```

### Do you require a little more control?
* instantiate a config and update it
```rs
use echo_http::Echo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // your instance must be mutable if you intend to update your config
    let mut echo = Echo::configure(None);
    let mut headers = Headers::new();
    headers.insert_many(vec![
        "Content-Type: application/json",
        "Authorization: Bearer token",
    ]);
    echo.headers = Some(headers.clone());

    let posts = echo
        .get<Vec<Post>>("https://jsonplaceholder.typicode.com/posts")
        .await?
        .data // you can chain .data to get the Response.data directly. cool, right?

    for post in posts {
        println!("Post ID: {}, Title: {}", post.id, post.title);
    }
}
```
* or just create one with your preferences if you dont intend on updating it later
```rs
use echo_http::{Echo, RequestConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = RequestConfig::default();
    config.base_url = Some("https://my_backend.api".to_string());
    config.timeout = Some(5000);
    
    let echo = Echo::configure(Some(config));
    
    // base_url is already defined, we can just add an endpoint here
    let users = echo.get<User>("/users/1") 
    
    ....
}
```

* post requets 
```rs
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Post {
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    let echo = Echo::configure(Some(/* set base_url */));

    let new_post = Post {
        user_id: 1,
        id: 1,
        title: "post title".to_string(),
        body: "compelling post body".to_string(),
    };

    // since the base_url is already set, we can just add the endpoint here
    let res = echo.post::<Post>("posts", Some(new_post)).await?;
    println!("{:#?}", res);

    Ok(())
}
```

* put example
```rs
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Post {
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    let echo = Echo::configure(Some(/* set base_url */));

    let updated_post = Post {
        user_id: 1,
        id: 1,
        title: "updated post title".to_string(),
        body: "compelling post body with the classic reddit *edit:".to_string(),
    };

    // since the base_url is already set, we can just add the endpoint here
    let res = echo.post::<Post>("posts", Some(updated_post)).await?;
    println!("{:#?}", res);

    Ok(())
}
```

* delete example - does not take a type argument
```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let echo = Echo::configure(None);
    let res = echo.delete("https://jsonplaceholder.typicode.com/posts/1").await?;
    println!("{:#?}", res);
    Ok(())
}
```

##### contributing: if you want to?
