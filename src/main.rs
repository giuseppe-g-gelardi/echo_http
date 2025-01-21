
use echo_http::Echo;
// use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hey bin main lol");

    let echo = Echo::configure(None);

    let res_headers = echo.get("https://jsonplaceholder.typicode.com/posts/1").await?;

    // let res_json = json!(res_headers);

    println!("{:?}", res_headers);


    Ok(())
}
