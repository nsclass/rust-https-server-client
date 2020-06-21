#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get("https://localhost:8088/")
        .send()
        .await?;

    println!("res: is {}", res.status());

    Ok(())
}
