# echo_http

`echo_http` is a type-safe HTTP client library built with Rust, based on Axios, for the typescript devs that just can't let go

## Features

- Type-safe HTTP requests and responses
- Async support with `reqwest`
- Generic response deserialization

## Getting started
* In addition to echo_http, you need an async runtime. I suggest tokio.
* it is suggested to also use reqwest as some functionality, like HeaderMaps, require it. 
* You will also very likely need serde to serialize/deserialize

## Example Usage

### In a hurry?
just use `echo_http::echo;`
* This is a static default instance. No need to configure anything, just bring into scope and go!
* works with (as of now) get, post, put and delete
```rs
use echo_http::echo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = echo.get<Vec<Post>>("https://jsonplaceholder.typicode.com/posts").await?;
    println!("{}", res.data);
    Ok(())
}
```

### Have no idea what data type youre expecting?
* use the `get_unknown()` method. This method does not take a type argument and returns a serde_json::Value in place of echo_http::Response.data / Response<T>. I'm probably not explaining this correctly, forgive me.
* Only supports GET requests
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
    // your instance mut be mutable if you intend to update your config
    let mut echo = Echo::configure(None);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", "Bearer token".parse().unwrap());
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

### yes. i know all the examples thus far are get requests.
* TODO: examples with put and delete

##### contributing: if you want to?
