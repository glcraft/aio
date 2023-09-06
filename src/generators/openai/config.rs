use serde::{Deserialize, Serialize};
use super::ChatRequestParameters;
use super::{Message, Role};
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub prompts: Vec<Prompt>,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Parameters {
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
    stop: Option<String>,
}
impl From<Parameters> for ChatRequestParameters {
    fn from(parameters: Parameters) -> Self {
        Self {
            max_tokens: parameters.max_tokens,
            temperature: parameters.temperature,
            top_p: parameters.top_p,
            presence_penalty: parameters.presence_penalty,
            frequency_penalty: parameters.frequency_penalty,
            best_of: parameters.best_of,
            n: parameters.n,
            stop: parameters.stop,
            ..Default::default()
        }
    }
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Prompt {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub messages: Vec<Message>,
    pub parameters: Parameters,
}

impl Prompt {
    pub fn from_input(input: &str) -> Self {
        Self {
            name: "noname".to_string(),
            messages: vec![Message {
                role: super::Role::User,
                content: input.into(),
            }],
            ..Default::default()
        }
    }
    pub fn format_contents(mut self, args: &crate::args::ProcessedArgs) -> Self {
        self.messages.iter_mut().map(|m| m.format_content_as_ref(&args)).for_each(|_| ());
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prompts: vec![
                Prompt {
                    name: "command".to_string(),
                    model: None,
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
                    model: None,
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
            ],
        }
    }
}