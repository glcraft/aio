use std::str::FromStr;

use serde::{Serialize, Deserialize, de::Error};

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

#[derive(Debug, Serialize)]
struct Message {
    role: Role,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_of: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<String>,
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
    pub fn into_stream(mut self) -> Self {
        self.stream = Some(true);
        self
    }
}
impl Default for ChatRequest {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: Vec::new(),
            max_tokens: None,
            temperature: None,
            top_p: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Delta {
    pub role: Option<Role>,
    pub content: Option<String>
}
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub delta: Delta,
    pub index: u32,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ChatResponse {
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
                match (&choice.delta.role, &choice.delta.content) {
                    (Some(role), Some(content)) => write!(f, "\n{}: {}", role, content),
                    (Some(role), None) => write!(f, "\n{}: ", role),
                    (None, Some(content)) => write!(f, "{}", content),
                    (None, None) => Ok(()),
                }
            },
            ChatResponse::Done => {
                if cfg!(feature = "debug") {
                    write!(f, "\nStream finished")
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl FromStr for ChatResponse {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "data: [DONE]" {
            Ok(ChatResponse::Done)
        } else if s.starts_with("data: ") {
            serde_json::from_str::<ChatResponse>(&s[5..])
        } else {
            Err(serde_json::Error::custom("Not a data line"))
        }
    }
}