use serde::{Deserialize, Serialize};
use super::super::openai::{Message, Role};
use llama_cpp::{LlamaTokenizationError, Token};

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptTemplate {
    #[default]
    ChatML,
    Llama2,
    Llama3,
}

fn append_to_vec<T: Copy>(vec: &mut Vec<T>, other: &[T]) {
    vec.reserve(other.len());
    other.iter().for_each(|v| vec.push(*v));
}

impl PromptTemplate {
    pub fn name(&self) -> &str {
        match self {
            PromptTemplate::ChatML => "chatml",
            PromptTemplate::Llama2 => "llama2",
            PromptTemplate::Llama3 => "llama3",
        }
    }
    pub fn messages_to_tokens(&self, model: &llama_cpp::LlamaModel, prompt: &[Message]) -> Result<Vec<Token>, LlamaTokenizationError> {
        let mut tokens = Vec::new();
        tokens.push(model.bos());
        match self {
            Self::ChatML => Self::tokens_chatml(&mut tokens, model, prompt),
            Self::Llama2 => {
                todo!("not implemented")
                // let context = prompt.iter()
                //     .fold(String::new(), |mut str, m| {
                //         match m.role {
                //             Role::User => {
                //                 #[allow(clippy::write_with_newline)]
                //                 let _ = write!(str, "[INST] {} [/INST]\n", format_content(&m.content, args));
                //             }
                //             Role::Assistant => {
                //                 #[allow(clippy::write_with_newline)]
                //                 let _ = write!(str, "{}</s>\n", format_content(&m.content, args));
                //             }
                //             _ => ()
                //         }
                //         str
                //     });
                // format!("<s>{}", context)
            }
            Self::Llama3 => Self::tokens_llama3(&mut tokens, model, prompt),
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
}

