## Anthropic SDK for Rust

The Anthropic SDK for Rust provides a simple and efficient way to interact with the Anthropic API, allowing you to send requests and process responses asynchronously. This SDK supports both streaming and non-streaming responses, making it versatile for different use cases.

### Features ✨

- Asynchronous request handling 🚀
- Support for streaming API responses 🌊
- Easy configuration for authentication and request parameters 🔧
- Error handling with detailed messages 🛠️💬

### Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
anthropic_sdk = "0.1.0"
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
        .auth(secret_key.as_str())
        .model("claude-3-opus-20240229")
        .messages(&json!([
            {"role": "user", "content": "Write me a poem about bravery"}
        ]))
        .max_tokens(1024)
        .build()?;

    if let Err(error) = request.execute(|text| println!("{text}")).await {
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
```

In this example, the response is processed as it streams in. Each chunk of text is printed as soon as it's received, and the entire message is also available at the end.

### Configuration

Before running the examples, make sure to set your Anthropic API key in an `.env` file at the root of your project:

```
ANTHROPIC_API_KEY=your_api_key_here
```

Alternatively, you can set the `ANTHROPIC_API_KEY` environment variable in your system.


### Feature Requests 📬

If you have any ideas or requests for new features, we'd love to hear from you! Please send your suggestions to [hello@mixpeal.com](mailto:hello@mixpeal.com), and we'll be sure to consider them for future updates. Your input is invaluable in helping us improve! 🌟🚀


### Error Handling

The SDK provides detailed error messages for various failure scenarios, such as network issues, invalid parameters, or unauthorized access. Errors are returned as `Result` types, allowing you to handle them using idiomatic Rust error handling patterns.
