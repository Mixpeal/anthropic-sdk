## Anthropic SDK for Rust

The Anthropic SDK for Rust provides a simple and efficient way to interact with the Anthropic API, allowing you to send requests and process responses asynchronously. This SDK supports both streaming and non-streaming responses, making it versatile for different use cases.

### Features ✨

- Asynchronous request handling 🚀
- Support for streaming API responses 🌊
- Easy configuration for authentication and request parameters 🔧
- Error handling with detailed messages 🛠️💬
- Optional verbose mode for raw response output 📃

### Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
anthropic_sdk = "0.1.5"
dotenv = "0.15.0"
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Usage

#### Basic Usage

For non-streaming responses, you can use the SDK as follows:

```rust
// examples/basic_usage.rs

use anthropic_sdk::Client;
use dotenv::dotenv;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let secret_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_default();

    let request = Client::new()
        .version("2023-06-01")
        .auth(secret_key.as_str())
        .model("claude-3-opus-20240229")
        .messages(&json!([
            {"role": "user", "content": "Write me a poem about bravery"}
        ]))
        .max_tokens(1024)
        // .verbose(true) // Uncomment to return the response as it is from Anthropic
        .build()?;

    if let Err(error) = request
        .execute(|text| async move { println!("{text}") })
        .await
    {
        eprintln!("Error: {error}");
    }

    Ok(())
}
```

This example demonstrates how to send a request to the Anthropic API and print the response.

#### Streaming Usage

For streaming responses, the SDK can be used as follows:

```rust
// examples/streaming_usage.rs

use anthropic_sdk::Client;
use dotenv::dotenv;
use serde_json::json;
use std::sync::{Arc, Mutex};

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
                }
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
```

In this example, the response is processed as it streams in. Each chunk of text is printed as soon as it's received, and the entire message is also available at the end.

#### Tool Use

For using Anthropic `Tool use`, the SDK can be used as follows:

```rust
// examples/tool_use_usage.rs

use anthropic_sdk::{Client, ToolChoice};
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
        .tool_choice(ToolChoice::Auto)
        .messages(&json!([
          {
            "role": "user",
            "content": "What is the weather like in San Francisco?"
          }
        ]))
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
```

This example demonstrates how to use tools with the Anthropic API to perform specific tasks, such as getting the weather.

### Fields Explanation

- `version`: (Optional) Specifies the version of the API to use.
- `auth`: Sets the authentication token for the API.
- `model`: Defines the model to use for generating responses.
- `messages`: Contains the input messages for the API to process.
- `max_tokens`: (Optional) Limits the number of tokens in the response.
- `stream`: (Optional) Enables streaming mode for receiving responses in real-time.
- `temperature`: (Optional) Adjusts the randomness of the response generation.
- `system`: (Optional) Provides additional context or instructions for the response.
- `tools`: (Optional) Specifies tools to use for specialized tasks.
- `tool_choice`: (Optional) Specifies the tool to use when multiple tools are available.
- `verbose`: (Optional) When set to true, returns the raw response from the API.
- `metadata`: (Optional) Includes additional information about the request.
- `stop_sequences`: (Optional) Specifies sequences where the API should stop generating further tokens.
- `top_k`: (Optional) Limits the model to only sample from the top K most likely next tokens.
- `top_p`: (Optional) Uses nucleus sampling to limit the model to a cumulative probability.

### Configuration

Before running the examples, make sure to set your Anthropic API key in an `.env` file at the root of your project:

```
ANTHROPIC_API_KEY=your_api_key_here
```

Alternatively, you can set the `ANTHROPIC_API_KEY` environment variable in your system.

### Feature Requests 📬

If you have any ideas or requests for new features, we'd love to hear from you! Please send your suggestions to [hello@mixpeal.com](mailto:hello@mixpeal.com), and we'll be sure to consider them for future updates. Your input is invaluable in helping us improve! 🌟🚀
