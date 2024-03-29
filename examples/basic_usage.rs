// examples/basic_usage.rs

use anthropic_sdk::Client;
use dotenv::dotenv;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let secret_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_default();

    let request = Client::new()
        .auth(secret_key.as_str())
        .model("claude-3-opus-20240229")
        .messages(&json!([
            {"role": "user", "content": "Write me a poem about bravery"}
        ]))
        .max_tokens(1024)
        .build()?;

    if let Err(error) = request
        .execute(|text| async move {
            println!("{text}");
        })
        .await
    {
        eprintln!("Error: {error}");
    }

    Ok(())
}
