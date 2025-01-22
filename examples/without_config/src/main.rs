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
    // let unknown = get_unknown().await?;
    // println!("{:#?}", unknown.data);

    let posts_response = echo
        .get::<Vec<Post>>("https://jsonplaceholder.typicode.com/posts")
        .await?;

    for post in posts_response.data {
        println!("Title: {}, ID: {}", post.title, post.id)
    }

    // println!("{:#?}", posts_response.data);
    //
    // let posts_vec = posts_response.data;
    // posts_vec.as_array().unwrap().iter().for_each(|post| {
    //     println!("Title: {}", post.get("title").unwrap());
    // });

    Ok(())
}

// async fn get_unknown() -> Result<Response, Err> {
//     let res = echo.get_unknown("https://httpbin.org/get").await?;
//     Ok(res)
// }
