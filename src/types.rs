use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnthropicUsage {
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnthropicContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnthropicTextDelta {
    #[serde(rename = "type")]
    pub delta_type: Option<String>,
    pub text: Option<String>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Option<AnthropicUsage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnthropicMessage {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub role: Option<String>,
    pub content: Option<Vec<AnthropicContentBlock>>,
    pub model: Option<String>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Option<AnthropicUsage>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct AnthropicChatCompletionChunk {
    #[serde(rename = "type")]
    pub event_type: String,
    pub index: Option<usize>,
    pub delta: Option<AnthropicTextDelta>,
    pub message: Option<AnthropicMessage>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnthropicErrorMessage {
    #[serde(rename = "type")]
    pub error_type: String,
    pub error: AnthropicErrorDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnthropicErrorDetails {
    pub details: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

#[derive(Debug)]
pub enum ToolChoice {
    Auto,
    Any,
    Tool(String),
}

impl Serialize for ToolChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ToolChoice::Auto => serde::Serialize::serialize(&serde_json::json!({"type": "auto"}), serializer),
            ToolChoice::Any => serde::Serialize::serialize(&serde_json::json!({"type": "any"}), serializer),
            ToolChoice::Tool(name) => serde::Serialize::serialize(&serde_json::json!({"type": "tool", "name": name}), serializer),
        }
    }
}