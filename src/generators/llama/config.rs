use serde::{Deserialize, Serialize};
use llama_cpp::standard_sampler::StandardSampler;
use crate::config::prompt::Message;
use super::template::PromptTemplate;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
    pub models: Vec<Model>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub template: PromptTemplate,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Prompt {
    pub name: String,
    pub content: Vec<Message>,
    pub parameters: PromptParameters
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PromptParameters {
    pub n_prev_tokens: i32,
    pub top_k: i32,
    pub top_p: f32,
    pub temperature: f32,
    pub repeat_penalty: f32,
    pub repeat_last_n: i32,
    pub max_tokens: i32,
    pub negative_prompt: Option<String>,
}

impl From<PromptParameters> for StandardSampler {
    fn from(parameters: PromptParameters) -> Self {
        Self {
            n_prev: parameters.n_prev_tokens,
            top_k: parameters.top_k,
            top_p: parameters.top_p,
            temp: parameters.temperature,
            penalty_repeat: parameters.repeat_penalty,
            penalty_last_n: parameters.repeat_last_n,
            cfg_negative_prompt: parameters.negative_prompt.unwrap_or_default(),
            ..Default::default()
        }
    }
}
impl Default for PromptParameters {
    fn default() -> Self {
        let default_standard_sampler = StandardSampler::default();
        Self {
            max_tokens: 1000,
            n_prev_tokens: default_standard_sampler.n_prev,
            top_k: default_standard_sampler.top_k,
            top_p: default_standard_sampler.top_p,
            temperature: default_standard_sampler.temp,
            repeat_penalty: default_standard_sampler.penalty_repeat,
            repeat_last_n: default_standard_sampler.penalty_last_n,
            negative_prompt: None,
        }
    }
}