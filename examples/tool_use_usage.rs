// examples/tool_use_usage.rs

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
        .beta("tools-2024-04-04") // add the beta header
        .tools(&json!([
          {
            "name": "get_weather",
            "description": "Get the current weather in a given location",
            "input_schema": {
              "type": "object",
              "properties": {
                "location": {
                  "type": "string",
                  "description": "The city and state, e.g. San Francisco, CA"
                }
              },
              "required": ["location"]
            }
          }
        ]))
        .messages(&json!([
          {
            "role": "user",
            "content": "What is the weather like in San Francisco?"
          }
        ]))
        .metadata(&json!({"user_id": "111"}))
        .max_tokens(1024)
        .build()?;

    if let Err(error) = request
        .execute(|text| async move { println!("{text}") })
        .await
    {
        eprintln!("Error: {error}");
    }

    Ok(())
}
