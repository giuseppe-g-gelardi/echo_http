use echo_http::{Config, Echo};

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

    let response = config_withurl.get("/users/1").await?;

    println!("this is with the config: {:#?}", response.data);

    Ok(())
}
