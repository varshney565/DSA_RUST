use serde_json::{json, Value};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "getOne",
        "params": [],
        "id": 1
    });
    let response = client
        .post("http://localhost:3039")
        .json(&request_body)
        .send()
        .await?;
    let response_json: Value = response.json().await?;
    println!("Response: {:?}", response_json);
    Ok(())
}
