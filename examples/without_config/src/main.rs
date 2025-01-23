use echo_http::echo;
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
    let unknown = echo.get_unknown("https://httpbin.org/get").await?;
    println!("{:#?}", unknown.data);

    let posts = echo
        .get::<Vec<Post>>("https://jsonplaceholder.typicode.com/posts")
        .await?
        .data; // you can chain .data to get the Response.data directly. cool, right?

    for post in posts {
        println!("Title: {}, ID: {}", post.title, post.id)
    }

    let deleted = echo
        .delete("https://jsonplaceholder.typicode.com/posts/1")
        .await?;

    println!("{:#?}", deleted);

    Ok(())
}
