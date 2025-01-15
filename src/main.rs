use echo_http::Axios;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a_some = Axios::configure(Some("github.com/".to_string()));
    let res = a_some.get("users".to_string()).await;
    println!("{}", res);

    let a_none = Axios::configure(None);
    let r = a_none.get("http://localhost:3000/api/dev/users/123".to_string()).await;
    println!("{}", r);

    Ok(())
}
