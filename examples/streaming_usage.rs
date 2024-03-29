// examples/streaming_usage.rs

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
        .stream(true)
        .build()?;

    let mut message = String::new();

    if let Err(error) = request
        .execute(|text| {
            println!("{text}"); // use the stream at this point
            message = format!("{message}{text}");
        })
        .await
    {
        eprintln!("Error: {error}");
    }

    println!("Message: {message}"); // or get the whole message at the end

    Ok(())
}
