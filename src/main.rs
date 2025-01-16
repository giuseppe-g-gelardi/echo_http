use echo_http::{Config, Echo, Nope};
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let a_some = Echo::configure(Some("github.com/".to_string()));
    // let res = a_some.get("users".to_string()).await;
    // println!("{}", res);
    //
    // let a_none = Echo::configure(None);
    // let r = a_none.get("http://localhost:3000/api/dev/users/123".to_string()).await;
    // println!("{}", r);
    //
    // println!("main");
    //
    // let config = Echo::configure(None);
    //
    // let res = config
    //     .get("https://jsonplaceholder.typicode.com/users/1")
    //     .await?;
    //
    // println!("this is the res???? {:#?}", res);

    let echo_config = Config {
        base_url: Some("https://jsonplaceholder.typicode.com/".to_string()),
        timeout: None,
        headers: None,
    };

    let config_withurl = Echo::configure(Some(echo_config));

    // let response = config_withurl.get("/users/1").await?;

    let post_res = config_withurl.post("/users", Nope).await?;

    // println!("this is with the config: {:#?}", response.data);

    println!("post res??????: {:#?}", post_res.status);

    #[derive(Debug, Serialize)]
    struct User {
        pub name: String,
        pub job: String,
    }

    let new_user = User {
        name: "first_name".to_string(),
        job: "shitty software dev".to_string(),
    };

    let echo = Echo::configure(None);
    let res = echo
        .post::<User>("https://reqres.in/api/users", Some(new_user))
        .await?;

    println!("RES BRO: {:#?}", res);
    Ok(())
}

//
// {
//   "name": "morpheus",
//   "job": "leader"
// }
//
//
//
// {
//   "name": "morpheus",
//   "job": "leader",
//   "id": "123",
//   "createdAt": "2025-01-16T10:00:00.000Z"
// }
