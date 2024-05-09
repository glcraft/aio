use std::collections::HashMap;

use llama_cpp::standard_sampler::StandardSampler;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Prompts(pub Vec<Prompt>);

impl Prompts {
    pub fn format_contents(mut self, args: &HashMap<String, String>) -> Self {
        self.0.iter_mut().for_each(|v| { Prompt::format_contents_as_ref(v, args); });
        self
    }
    pub fn format_contents_as_ref(&mut self, args: &HashMap<String, String>) -> &mut Self {
        self.0.iter_mut().for_each(|v| { Prompt::format_contents_as_ref(v, args); });
        self
    }
}
impl Default for Prompts {
    fn default() -> Self {
        Prompts(vec![
                Prompt {
                    name: "command".to_string(),
                    messages: vec![
                        Message {
                            role: Role::System,
                            content: Some("In markdown, write the unix command that best fits my request in a block of code under a \"## Command\" then describe the program and each parameter in \"## Explanation\".".to_string()),
                        },
                        Message {
                            role: Role::User,
                            content: Some("$input".to_string()),
                        },
                        Message {
                            role: Role::Assistant,
                            content: None,
                        },
                    ],
                    parameters: Parameters {
                        max_tokens: Some(200),
                        temperature: Some(0.0),
                        top_p: Some(1.0),
                        presence_penalty: Some(0.0),
                        frequency_penalty: Some(0.2),
                        ..Default::default()
                    },
                },
                Prompt {
                    name: "ask".to_string(),
                    messages: vec![
                        Message {
                            role: Role::System,
                            content: Some("You are a powerful intelligent conversational chatbot. Unless I tell you otherwise, answer to me in an informative way. You should format the text in Markdown.".to_string()),
                        },
                        Message {
                            role: Role::User,
                            content: Some("$input".to_string()),
                        },
                        Message {
                            role: Role::Assistant,
                            content: None,
                        },
                    ],
                    parameters: Parameters {
                        max_tokens: Some(300),
                        temperature: Some(0.7),
                        top_p: Some(1.0),
                        presence_penalty: Some(0.0),
                        frequency_penalty: Some(0.0),
                        ..Default::default()
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
                content: Some(input.into()),
            }],
            ..Default::default()
        }
    }
    pub fn format_contents(mut self, args: &HashMap<String, String>) -> Self {
        self.messages.iter_mut().for_each(|m|{ m.format_content_as_ref(args); });
        self
    }
    pub fn format_contents_as_ref(&mut self, args: &HashMap<String, String>) -> &mut Self {
        self.messages.iter_mut().for_each(|m| { m.format_content_as_ref(args); });
        self
    }
    pub fn formatted_messages(&self, args: &HashMap<String, String>) -> Vec<Message> {
        self.messages.iter().cloned().map(|v| Message::format_content(v, args)).collect()
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
    pub content: Option<String>,
}

#[allow(dead_code)]
impl Message {
    pub fn format_content(mut self, args: &HashMap<String, String>) -> Self {
        self.content = self.content.map(|c| crate::config::format_content(&c, args).to_string());
        self
    }
    pub fn format_content_as_ref(&mut self, args: &HashMap<String, String>) -> &mut Self {
        self.content = self.content.as_mut().map(|c| crate::config::format_content(c, args).to_string());
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
    #[serde(skip_serializing_if = "Stop::is_none")]
    #[serde(default)]
    pub stop: Stop,
    
    //OpenAI only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    //Local only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_prev_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
}

impl From<Parameters> for StandardSampler {
    fn from(parameters: Parameters) -> Self {
        let def = StandardSampler::default();
        StandardSampler {
            temp: parameters.temperature.unwrap_or(def.temp),
            top_p: parameters.top_p.unwrap_or(def.top_p),
            penalty_repeat: parameters.presence_penalty.unwrap_or(def.penalty_repeat),
            penalty_freq: parameters.frequency_penalty.unwrap_or(def.penalty_freq),
            n_prev: parameters.n_prev_tokens.unwrap_or(def.n_prev as _) as _,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Stop {
    #[default]
    None,
    #[serde(untagged)]
    One(String),
    #[serde(untagged)]
    Many(Vec<String>),
}

impl Stop {
    pub fn is_none(&self) -> bool {
        matches!(self, Stop::None)
    }
}