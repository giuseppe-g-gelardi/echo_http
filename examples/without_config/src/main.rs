use echo_http::{echo, Response, ResponseUnknown};
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
    let unknown = get_unknown().await?;
    println!("{:#?}", unknown.data);

    let posts = get().await?.data;

    for post in posts {
        println!("ID: {}", post.id)
    }

    let new_post = post().await?;
    println!("{:#?}", new_post.data);

    let updated_post = put().await?;
    println!("{:#?}", updated_post.data);

    let deleted = delete().await?;
    println!("{:#?}", deleted.data); // data should return an empty object

    Ok(())
}

async fn get_unknown() -> Result<ResponseUnknown, Err> {
    let unknown = echo.get_unknown("https://httpbin.org/get").await?;
    Ok(unknown)
}

async fn get() -> Result<Response<Vec<Post>>, Err> {
    let posts = echo
        .get::<Vec<Post>>("https://jsonplaceholder.typicode.com/posts")
        .await?;

    Ok(posts)
}

async fn post() -> Result<Response<Post>, Err> {
    let new_post = Post {
        user_id: 1,
        id: 1,
        title: "post title".to_string(),
        body: "compelling post body".to_string(),
    };

    let posted = echo
        .post("https://jsonplaceholder.typicode.com/posts", Some(new_post))
        .await?;

    Ok(posted)
}

async fn put() -> Result<Response<Post>, Err> {
    let updated_post = Post {
        user_id: 1,
        id: 1,
        title: "post title".to_string(),
        body: "compelling post body".to_string(),
    };

    let put = echo
        .put(
            "https://jsonplaceholder.typicode.com/posts/1",
            Some(updated_post),
        )
        .await?;

    Ok(put)
}

async fn delete() -> Result<ResponseUnknown, Err> {
    let deleted = echo
        .delete("https://jsonplaceholder.typicode.com/posts/1")
        .await?;

    Ok(deleted)
}
