use anyhow::{anyhow, Context, Result};
use reqwest::{Client as ReqwestClient, Error as ReqwestError, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use types::AnthropicChatCompletionChunk;
mod types;

#[derive(Serialize)]
struct ApiRequestBody<'a> {
    model: &'a str,
    max_tokens: i32,
    messages: &'a Value,
    stream: bool,
    temperature: f32,
    system: &'a str,
}

pub struct Client {
    client: ReqwestClient,
    secret_key: String,
    model: String,
    messages: Value,
    max_tokens: i32,
    stream: bool,
    temperature: f32,
    system: String,
}

#[derive(Deserialize)]
struct JsonResponse {
    content: Vec<Content>,
}

#[derive(Deserialize)]
struct Content {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
            secret_key: String::new(),
            model: String::new(),
            messages: Value::Null,
            max_tokens: 1024,
            stream: false,
            temperature: 0.0,
            system: String::new(),
        }
    }

    pub fn auth(mut self, secret_key: &str) -> Self {
        self.secret_key = secret_key.to_owned();
        self
    }

    pub fn model(mut self, model: &str) -> Self {
        self.model = model.to_owned();
        self
    }

    pub fn messages(mut self, messages: &Value) -> Self {
        self.messages = messages.clone();
        self
    }

    pub fn max_tokens(mut self, max_tokens: i32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.to_owned();
        self
    }

    pub fn system(mut self, system: &str) -> Self {
        self.system = system.to_owned();
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }

    pub fn build(self) -> Result<Request, ReqwestError> {
        let body = ApiRequestBody {
            model: &self.model,
            max_tokens: self.max_tokens,
            messages: &self.messages,
            stream: self.stream,
            temperature: self.temperature,
            system: &self.system,
        };

        let request_builder = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", self.secret_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body);

        Ok(Request {
            request_builder,
            stream: self.stream,
        })
    }
}

pub struct Request {
    request_builder: RequestBuilder,
    stream: bool,
}

impl Request {
    pub async fn execute<F>(self, mut callback: F) -> Result<()>
    where
        F: FnMut(&str) + Send + 'static,
    {
        let mut response = self
            .request_builder
            .send()
            .await
            .context("Failed to send request")?;

        match response.status() {
            StatusCode::OK => {
                if self.stream {
                    let mut buffer = String::new();
                    while let Some(chunk) = response.chunk().await? {
                        let s = match std::str::from_utf8(&chunk) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };
                        buffer.push_str(s);
                        loop {
                            if let Some(index) = buffer.find("\n\n") {
                                let chunk = buffer[..index].to_string();
                                buffer.drain(..=index + 1);

                                if chunk == "data: [DONE]" {
                                    break;
                                }
                                let processed_chunk = chunk
                                    .trim_start_matches("event: message_start")
                                    .trim_start_matches("event: content_block_start")
                                    .trim_start_matches("event: ping")
                                    .trim_start_matches("event: content_block_delta")
                                    .trim_start_matches("event: content_block_stop")
                                    .trim_start_matches("event: message_delta")
                                    .trim_start_matches("event: message_stop")
                                    .to_string();
                                let cleaned_string = &processed_chunk
                                    .trim_start()
                                    .strip_prefix("data: ")
                                    .unwrap_or(&processed_chunk);
                                match serde_json::from_str::<AnthropicChatCompletionChunk>(
                                    &cleaned_string,
                                ) {
                                    Ok(d) => {
                                        if let Some(delta) = d.delta {
                                            if let Some(content) = delta.text {
                                                callback(&content);
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        println!(
                                            "Couldn't parse AnthropicChatCompletionChunk: {}",
                                            &cleaned_string
                                        );
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    let json_text = response
                        .text()
                        .await
                        .context("Failed to read response text")?;
                    match serde_json::from_str::<JsonResponse>(&json_text) {
                        Ok(parsed_json) => {
                            if let Some(content) = parsed_json
                                .content
                                .iter()
                                .find(|c| c.content_type == "text")
                            {
                                callback(&content.text);
                            }
                        }
                        Err(_) => return Err(anyhow!("Unable to parse JSON")),
                    }
                }
                Ok(())
            }
            StatusCode::BAD_REQUEST => Err(anyhow!("Bad request. Check your request parameters.")),
            StatusCode::UNAUTHORIZED => Err(anyhow!("Unauthorized. Check your authorization.")),
            _ => {
                let error_message = format!("Unexpected status code: {:?}", response.status());
                Err(anyhow!(error_message))
            }
        }
    }
}
