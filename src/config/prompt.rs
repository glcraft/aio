use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Prompts(pub Vec<Prompt>);

impl Default for Prompts {
    fn default() -> Self {
        Prompts(vec![
                Prompt {
                    name: "command".to_string(),
                    messages: vec![
                        Message {
                            role: Role::System,
                            content: "In markdown, write the command that best fits my request in a \"Nu\" block in \"## Command\" then describe each parameter in \"## Explanation\".".to_string(),
                        },
                        Message {
                            role: Role::User,
                            content: "$input".to_string(),
                        },
                    ],
                    parameters: Parameters {
                        max_tokens: Some(200),
                        temperature: Some(0.0),
                        top_p: Some(1.0),
                        presence_penalty: Some(0.0),
                        frequency_penalty: Some(0.2),
                        best_of: None,
                        n: None,
                        stop: None,
                    },
                },
                Prompt {
                    name: "ask".to_string(),
                    messages: vec![
                        Message {
                            role: Role::System,
                            content: "You are ChatGPT, a powerful conversational chatbot. Answer to me in informative way unless I tell you otherwise. Format the text in markdown.".to_string(),
                        },
                        Message {
                            role: Role::User,
                            content: "$input".to_string(),
                        },
                    ],
                    parameters: Parameters {
                        max_tokens: Some(300),
                        temperature: Some(0.7),
                        top_p: Some(1.0),
                        presence_penalty: Some(0.0),
                        frequency_penalty: Some(0.0),
                        best_of: None,
                        n: None,
                        stop: None,
                    },
                },
            ])
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Prompt {
    pub name: String,
    pub messages: Vec<Message>,
    pub parameters: Parameters,
}

impl Prompt {
    pub fn from_input(input: &str) -> Self {
        Self {
            name: "noname".to_string(),
            messages: vec![Message {
                role: Role::User,
                content: input.into(),
            }],
            ..Default::default()
        }
    }
    pub fn format_contents(mut self, args: &HashMap<String, String>) -> Self {
        self.messages.iter_mut().map(|m| m.format_content_as_ref(args)).for_each(|_| ());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
impl Role {
    pub fn lowercase(&self) -> &str {
        match self {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[allow(dead_code)]
impl Message {
    pub fn format_content(mut self, args: &HashMap<String, String>) -> Self {
        self.content = crate::config::format_content(&self.content, args).to_string();
        self
    }
    pub fn format_content_as_ref(&mut self, args: &HashMap<String, String>) -> &mut Self {
        self.content = crate::config::format_content(&self.content, args).to_string();
        self
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
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
    pub stop: Option<String>,
}



