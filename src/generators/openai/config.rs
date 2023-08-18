use serde::Deserialize;
use super::ChatRequestParameters;
use super::Message;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub prompts: Vec<Prompt>,
}
#[derive(Debug, Deserialize)]
pub struct Parameters {
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    best_of: Option<u32>,
    n: Option<u32>,
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
#[derive(Debug, Deserialize)]
pub struct Prompt {
    pub name: String,
    pub messages: Vec<Message>,
    pub parameters: Parameters,
}

impl Prompt {
    pub fn format_contents(mut self, args: &crate::args::Args) -> Self {
        self.messages.iter_mut().map(|m| m.format_content_as_ref(&args)).for_each(|_| ());
        self
    }
}