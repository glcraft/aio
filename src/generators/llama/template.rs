use serde::{Deserialize, Serialize};
use super::super::openai::{Message, Role};
use llama_cpp::{LlamaTokenizationError, Token};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct CustomTemplate {
    pub system_prefix: String,
    pub system_suffix: String,
    pub user_prefix: String,
    pub user_suffix: String,
    pub assistant_prefix: String,
    pub assistant_suffix: String,
}
#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptTemplate {
    #[default]
    ChatML,
    Llama2,
    Llama3,
    Custom(CustomTemplate)
}

fn append_to_vec<T: Copy>(vec: &mut Vec<T>, other: &[T]) {
    vec.reserve(other.len());
    other.iter().for_each(|v| vec.push(*v));
}

macro_rules! vec_merge {
    ($tokens:ident, $($other_tokens:expr),*) => {{
        let arrs = [$($other_tokens),*];
        $tokens.reserve(arrs.iter().map(|arr| arr.len()).sum());
        arrs.iter().map(|arr| arr.iter()).flatten().for_each(|v| $tokens.push(*v));
    }};
}

impl PromptTemplate {
    pub fn name(&self) -> &str {
        match self {
            PromptTemplate::ChatML => "chatml",
            PromptTemplate::Llama2 => "llama2",
            PromptTemplate::Llama3 => "llama3",
            PromptTemplate::Custom(_) => "custom",
        }
    }
    pub fn messages_to_tokens(&self, model: &llama_cpp::LlamaModel, prompt: &[Message]) -> Result<Vec<Token>, LlamaTokenizationError> {
        let mut tokens = Vec::new();
        tokens.push(model.bos());
        match self {
            Self::ChatML => Self::tokens_chatml(&mut tokens, model, prompt),
            Self::Llama2 => Self::tokens_llama2(&mut tokens, model, prompt),
            Self::Llama3 => Self::tokens_llama3(&mut tokens, model, prompt),
            Self::Custom (custom_template) => Self::tokens_custom(&mut tokens, model, prompt, custom_template),
        }?;
        Ok(tokens)
    }
    pub fn tokens_chatml(tokens: &mut Vec<Token>, model: &llama_cpp::LlamaModel, prompt: &[Message]) -> Result<(), LlamaTokenizationError> {
        let im_start = model.tokenize_bytes("<|im_start|>", false, true)?.first().copied().unwrap();
        let im_end = model.tokenize_bytes("<|im_end|>", false, true)?.first().copied().unwrap();
        let [system, user, assistant] = [
            model.tokenize_bytes("system", false, true)?,
            model.tokenize_bytes("user", false, true)?,
            model.tokenize_bytes("assistant", false, true)?
        ];
        prompt.iter()
            .for_each(|m| {
                tokens.push(im_start);
                append_to_vec(tokens, match m.role {
                    Role::System => &system,
                    Role::User => &user,
                    Role::Assistant => &assistant
                });
                tokens.push(model.nl());
                append_to_vec(tokens, &model.tokenize_bytes(&m.content, false, false).unwrap());
                tokens.push(im_end);
                tokens.push(model.nl());
            });
        tokens.push(im_start);
        append_to_vec(tokens, &assistant);
        tokens.push(im_end);
        Ok(())
    }
    pub fn tokens_llama2(tokens: &mut Vec<Token>, model: &llama_cpp::LlamaModel, prompt: &[Message]) -> Result<(), LlamaTokenizationError> {
        let system_start = model.tokenize_bytes("<<SYS>>", false, true)?;
        let system_end = model.tokenize_bytes("<</SYS>>", false, true)?;
        let inst_start = model.tokenize_bytes("[INST]", false, true)?;
        let inst_end = model.tokenize_bytes("[/INST]", false, true)?;
        let eos = model.tokenize_bytes("</s>", false, true)?;
        let nl = model.tokenize_bytes("\n", false, true)?;
        prompt.iter()
            .for_each(|m| {
                match m.role {
                    Role::System => vec_merge!(tokens, &inst_start, &system_start, &model.tokenize_bytes(&m.content, false, false).unwrap(), &system_end, &inst_end, &nl),
                    Role::User => vec_merge!(tokens, &inst_start, &model.tokenize_bytes(&m.content, false, false).unwrap(), &inst_end, &nl),
                    Role::Assistant => vec_merge!(tokens, &model.tokenize_bytes(&m.content, false, false).unwrap(), &eos, &nl),
                }
            });
        Ok(())
    }
    pub fn tokens_llama3(tokens: &mut Vec<Token>, model: &llama_cpp::LlamaModel, prompt: &[Message]) -> Result<(), LlamaTokenizationError> {
        let start_header_id = model.tokenize_bytes("<|start_header_id|>", false, true)?.first().copied().unwrap();
        let end_header_id = model.tokenize_bytes("<|end_header_id|>", false, true)?.first().copied().unwrap();
        let eot_id = model.tokenize_bytes("<|eot_id|>", false, true)?.first().copied().unwrap();
        let nl = model.tokenize_bytes("\n", false, true)?.first().copied().unwrap();
        let [system, user, assistant] = [
            model.tokenize_bytes("system", false, true)?,
            model.tokenize_bytes("user", false, true)?,
            model.tokenize_bytes("assistant", false, true)?
        ];
        prompt.iter()
            .for_each(|m| {
                tokens.push(start_header_id);
                append_to_vec(tokens, match m.role {
                    Role::System => &system,
                    Role::User => &user,
                    Role::Assistant => &assistant
                });
                append_to_vec(tokens, &[end_header_id, nl, nl]);
                append_to_vec(tokens, &model.tokenize_bytes(&m.content, false, false).unwrap());
                tokens.push(eot_id);
            });
        tokens.push(start_header_id);
        append_to_vec(tokens, &assistant);
        append_to_vec(tokens, &[end_header_id, nl, nl]);
        Ok(())
    }
    pub fn tokens_custom(tokens: &mut Vec<Token>, model: &llama_cpp::LlamaModel, prompt: &[Message], custom_template: &CustomTemplate) -> Result<(), LlamaTokenizationError> {
        let system_prefix_tokens = model.tokenize_bytes(&custom_template.system_prefix, false, true)?;
        let system_suffix_tokens = model.tokenize_bytes(&custom_template.system_suffix, false, true)?;
        let user_prefix_tokens = model.tokenize_bytes(&custom_template.user_prefix, false, true)?;
        let user_suffix_tokens = model.tokenize_bytes(&custom_template.user_suffix, false, true)?;
        let assistant_prefix_tokens = model.tokenize_bytes(&custom_template.assistant_prefix, false, true)?;
        let assistant_suffix_tokens = model.tokenize_bytes(&custom_template.assistant_suffix, false, true)?;
        prompt.iter()
            .for_each(|m| {
                let content_tokens = model.tokenize_bytes(&m.content, false, false).unwrap();
                match m.role {
                    Role::System => vec_merge!(tokens, &system_prefix_tokens, &content_tokens, &system_suffix_tokens),
                    Role::User => vec_merge!(tokens, &user_prefix_tokens, &content_tokens, &user_suffix_tokens),
                    Role::Assistant => vec_merge!(tokens, &assistant_prefix_tokens, &content_tokens, &assistant_suffix_tokens),
                }
            });
        Ok(())
    }
    pub fn stop_tokens(&self, model: &llama_cpp::LlamaModel) -> Result<Vec<Token>, LlamaTokenizationError> {
        match self {
            PromptTemplate::ChatML => {
                let im_end = model.tokenize_bytes("<|im_end|>", false, true)?.first().copied().unwrap();
                Ok(vec![im_end, model.eos()])
            },
            PromptTemplate::Llama2 => {
                let eot_id = model.tokenize_bytes("[INST]", false, true)?.first().copied().unwrap();
                Ok(vec![eot_id, model.eos()])
            },
            PromptTemplate::Llama3 => {
                let eot_id = model.tokenize_bytes("<|eot_id|>", false, true)?.first().copied().unwrap();
                Ok(vec![eot_id, model.eos()])
            },
            PromptTemplate::Custom(custom_template) => Ok(vec![model.tokenize_bytes(&custom_template.user_prefix, false, true)?.first().copied().unwrap()]),
        }
    }
}

