use echo_http::{echo, Response};
use serde::Serialize;

type Err = Box<dyn std::error::Error>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    let unknown = get_unknown().await?;
    println!("{:#?}", unknown.data);

    let posts_response = echo
        .get::<Vec<Post>>("https://jsonplaceholder.typicode.com/posts")
        .await?;

    println!("{:#?}", posts_response.data);

    let posts_vec = posts_response.data;
    posts_vec.as_array().unwrap().iter().for_each(|post| {
        println!("Title: {}", post.get("title").unwrap());
    });

    // let post_list = echo
    //     .get::<Response<Vec<Post>>>("https://jsonplaceholder.typicode.com/posts")
    //     .await?
    //     .data;
    //
    // for post in post_list {}

    Ok(())
}

async fn get_unknown() -> Result<Response, Err> {
    let res = echo.get_unknown("https://httpbin.org/get").await?;
    Ok(res)
}
