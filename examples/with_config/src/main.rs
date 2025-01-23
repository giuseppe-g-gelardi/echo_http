use echo_http::{Echo, RequestConfig};
use serde::{Deserialize, Serialize};

type Err = Box<dyn std::error::Error>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    let mut echo_config = RequestConfig::default();

    // set base_url
    echo_config.base_url = Some("https://jsonplaceholder.typicode.com/".to_string());

    // set headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    echo_config.headers = Some(headers);

    // create echo instance
    let echo = Echo::configure(Some(echo_config));

    // since the base_url is set, we can use relative paths and the base_url will be prepended
    let res = echo.get::<Post>("posts/1").await?;

    println!("{:#?}", res.data);

    Ok(())
}
