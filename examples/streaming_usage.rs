// examples/streaming_usage.rs

use anthropic_sdk::Client;
use dotenv::dotenv;
use serde_json::json;
use std::sync::{Arc, Mutex};
// use tokio::time::{sleep, Duration};

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
        .system("Make it sound like Edgar Allan Poe")
        .temperature(0.1)
        .max_tokens(1024)
        .stream(true)
        .build()?;

    let message = Arc::new(Mutex::new(String::new()));
    let message_clone = message.clone();

    if let Err(error) = request
        .execute(move |text| {
            let message_clone = message_clone.clone();
            async move {
                println!("{text}");

                {
                    let mut message = message_clone.lock().unwrap();
                    *message = format!("{}{}", *message, text);
                    drop(message);
                }
                // Mimic async process
                // sleep(Duration::from_millis(200)).await; 
            }
        })
        .await
    {
        eprintln!("Error: {error}");
    }

    let final_message = message.lock().unwrap();
    println!("Message: {}", *final_message); // or get the whole message at the end

    Ok(())
}
