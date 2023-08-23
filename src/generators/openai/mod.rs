pub mod config;
pub mod credentials;
mod flatten_stream;
use bytes::Bytes;
use flatten_stream::FlattenTrait;

use serde::{Serialize, Deserialize};
use tokio_stream::{Stream, StreamExt};
use crate::args;
use super::{ResultStream, ResultRun, Error, BoxedError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Assistant => write!(f, "Assistant"),
            Role::System => write!(f, "System"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn format_content(mut self, args: &crate::args::Args) -> Self {
        self.content = crate::config::format_content(&self.content, args).to_string();
        self
    }
    pub fn format_content_as_ref(&mut self, args: &crate::args::Args) -> &mut Self {
        self.content = crate::config::format_content(&self.content, args).to_string();
        self
    }
}
#[derive(Debug, Default, Serialize)]
pub struct ChatRequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(flatten)]
    parameters: ChatRequestParameters,
}
impl ChatRequest {
    pub fn new(model: String) -> Self {
        Self {
            model,
            ..Default::default()
        }
    }
    pub fn add_message(mut self, role: Role, content: String) -> Self {
        self.messages.push(Message { role, content: content.into() });
        self
    }
    pub fn add_messages(mut self, messages: Vec<Message>) -> Self {
        self.messages.extend(messages);
        self
    }
    pub fn set_parameters(mut self, parameters: ChatRequestParameters) -> Self {
        self.parameters = parameters;
        self
    }
    pub fn into_stream(mut self) -> Self {
        self.parameters.stream = Some(true);
        self
    }
}
impl Default for ChatRequest {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: Vec::new(),
            parameters: Default::default()
        }
    }
}


#[derive(Debug, Deserialize)]
struct Delta {
    pub role: Option<Role>,
    pub content: Option<String>
}
#[derive(Debug, Deserialize)]
struct Choice {
    pub delta: Delta,
    pub index: u32,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ChatResponse {
    Message{
        id: String,
        object: String,
        created: u64,
        model: String,
        choices: Vec<Choice>,
    },
    #[serde(rename = "[DONE]")]
    Done,
}

impl std::fmt::Display for ChatResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ChatResponse::Message{choices, ..} => {
                if choices.len() == 0 {
                    return Ok(());
                }
                let choice = &choices[0];
                if let Some(content) = choice.delta.content.as_ref() {
                    write!(f, "{}", content)?;
                }
                Ok(())
            },
            ChatResponse::Done => {
                if cfg!(feature = "debug") {
                    write!(f, "\n<<Stream finished>>")
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl ChatResponse {
    pub fn from_slice(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        // eprintln!("from_bytes|1: {}", String::from_utf8_lossy(&bytes));
        if !bytes.starts_with(b"data: ") {
            use serde::de::Error;
            return Err(serde_json::Error::custom("Not a data line"));
        }
        let bytes = &bytes[6..];
        // eprintln!("from_bytes|2: {}", String::from_utf8_lossy(&bytes));
        if bytes.starts_with(b"[DONE]") {
            return Ok(ChatResponse::Done);
        }
        let result = serde_json::from_slice(&bytes);
        result
    }
    #[inline]
    pub fn from_bytes(bytes: bytes::Bytes) -> Result<Self, serde_json::Error> {
        Self::from_slice(&bytes)
    }
}

struct SplitBytes<Sep> 
where 
    Sep: AsRef<[u8]>
{
    bytes: Bytes,
    separator: Sep,
    index: Option<usize>,
}

impl<Sep> SplitBytes<Sep> 
where
    Sep: AsRef<[u8]>
{
    fn new(bytes: Bytes, separator: Sep) -> Self {
        Self {
            bytes,
            separator,
            index: Some(0),
        }
    }
}

impl<Sep> Iterator for SplitBytes<Sep> 
where
    Sep: AsRef<[u8]>
{
    type Item = Bytes;
    fn next(&mut self) -> Option<Self::Item> {
        let separator = self.separator.as_ref();
        let index = self.index?;
        let bytes = self.bytes.slice(index..);
        let found = bytes
            .windows(separator.len())
            .find(|b| b == &separator);
        let slice_bytes = if let Some(found) = found {
            let end_selection = found.as_ptr() as usize - bytes.as_ptr() as usize;
            self.index = self.index.map(|i| i + end_selection + found.len());
            bytes.slice(..end_selection)
        } else {
            self.index = None;
            bytes
        };
        match slice_bytes.is_empty() {
            false => Some(slice_bytes),
            true => None,
        }
    }
}

pub async fn run(creds: credentials::Credentials, config: crate::config::Config, args: args::Args) -> ResultRun {
    let openai_api_key = creds.api_key;

    if openai_api_key.is_empty() {
        return Err(Error::Custom("OpenAI API key not found".into()));
    }

    let prompt = config.openai.prompts.into_iter()
        .find(|prompt| prompt.name == args.prompt)
        .ok_or(Error::Custom("Prompt not found".into()))?
        .format_contents(&args);

    // Send a request
    let chat_request = ChatRequest::new("gpt-3.5-turbo".to_string())
        .add_messages(prompt.messages)
        .set_parameters(prompt.parameters.into())
        .into_stream();

    let client = reqwest::Client::new();
    let stream = client.post("https://api.openai.com/v1/chat/completions")
        .header("User-Agent", cargo_info::user_agent!())
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&chat_request)
        .send()
        .await?
        .bytes_stream();

    let stream_string = stream
        .map(|input| -> Result<_, Error> {
            Ok(SplitBytes::new(input?, b"\n\n"))
        })
        .flatten_stream()
        .map(|v| {
            let v = v?;
            let chat_resp = ChatResponse::from_slice(&v);
            match chat_resp {
                Ok(resp) => Ok(resp),
                Err(e) => Err(Error::Boxed(Box::new(e)))
            }
        })
        .map_while(|resp| {
            match resp {
                Ok(msg @ ChatResponse::Message { .. }) => Some(Ok(msg.to_string())),
                Ok(ChatResponse::Done) => None,
                Err(e) => return Some(Err(e)),
            }
        });
    Ok(Box::pin(stream_string))
}