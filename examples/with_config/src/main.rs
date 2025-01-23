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
    let mut headers = reqwest::header::HeaderMap::new();

    // set base_url
    echo_config.base_url = Some("https://jsonplaceholder.typicode.com/".to_string());

    // set headers
    headers.insert("Content-Type", "application/json".parse().unwrap());
    echo_config.headers = Some(headers);

    // create echo instance
    let echo = Echo::configure(Some(echo_config));

    let new_post = Post {
        user_id: 1,
        id: 1,
        title: "post title".to_string(),
        body: "compelling post body".to_string(),
    };

    let res = echo.post::<Post>("posts", Some(new_post)).await?;

    println!("{:#?}", res);

    Ok(())
}
