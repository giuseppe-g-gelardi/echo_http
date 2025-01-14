use fetcher::Axios;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = Axios::configure("github.com/".to_string());
    let res = a.get("users".to_string());

    println!("{}", res);
    println!("main");

    Ok(())
}
