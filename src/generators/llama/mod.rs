pub mod config;
use tokio_stream::StreamExt;

use llama_cpp::{
    standard_sampler::StandardSampler, LlamaModel, LlamaParams, SessionParams, Token, TokensToStrings
};
use once_cell::sync::OnceCell;
use crate::{
    config::Config as AIOConfig,
    args
};
use super::{openai::{Message, Role}, Error, ResultRun};

static LOCAL_LLAMA: OnceCell<LlamaModel> = OnceCell::new();

fn init_model(config: &config::Model) -> Result<(), Error> {
    let model_options = LlamaParams {
        n_gpu_layers: 20000,
        ..Default::default()
    };
    let Ok(llama) = LlamaModel::load_from_file(
        &config.path,
        model_options,
    ) else {
        return Err(Error::Custom("Failed to load LLaMA model".into()))
    };
    LOCAL_LLAMA.set(llama).map_err(|_| Error::Custom("Failed to set LLaMA model".into()))
}

fn make_context(prompt: &[Message], template: config::PromptTemplate, args: &args::ProcessedArgs) -> String {
    use std::fmt::Write;
    use crate::config::format_content;
    match template {
        config::PromptTemplate::ChatML => {
            let mut context = prompt.iter()
                .fold(String::new(), |mut str, m| {
                    let _ = write!(str, "<|im_start|>{}\n{}<|im_end|>\n", m.role.lowercase(), format_content(&m.content, args));
                    str
                });
            #[allow(clippy::write_with_newline)]
            let _ = write!(context, "<|im_start|>assistant\n");
            context
        }
        config::PromptTemplate::Llama2 => {
            let context = prompt.iter()
                .fold(String::new(), |mut str, m| {
                    match m.role {
                        Role::User => {
                            #[allow(clippy::write_with_newline)]
                            let _ = write!(str, "[INST] {} [/INST]\n", format_content(&m.content, args));
                        }
                        Role::Assistant => {
                            #[allow(clippy::write_with_newline)]
                            let _ = write!(str, "{}</s>\n", format_content(&m.content, args));
                        }
                        _ => ()
                    }
                    str
                });
            format!("<s>{}", context)
        }
        config::PromptTemplate::Llama3 => {
            let context = prompt.iter()
                .fold(String::new(), |mut str, m| {
                    let _ = write!(str, "<|start_header_id|>{}<|end_header_id|>\n\n{}<|eot_id|>", m.role.lowercase(), format_content(&m.content, args));
                    str
                });
            format!("<|begin_of_text|>{}<|start_header_id|>assistant<|end_header_id|>\n\n", context)
        }
    }
}

pub async fn run(
    config: AIOConfig, 
    args: args::ProcessedArgs
) -> ResultRun {
    let model = args.engine
        .split(':')
        .nth(1)
        .ok_or_else(|| Error::Custom("Model missing in engine parameter".into()))?;
    let model_config = config.local.models.into_iter()
        .find(|c| c.name == model)
        .ok_or_else(|| Error::Custom("Model not found in config".into()))?;
    if LOCAL_LLAMA.get().is_none() {
        init_model(&model_config)?;
    }
    let model = LOCAL_LLAMA.get().unwrap();
    
    let session_params = SessionParams::default();
    let mut session = model.create_session(session_params).map_err(|_| Error::Custom("Failed to create session".into()))?;

    let context = make_context(&config.local.prompts.first().unwrap().content, model_config.template, &args);
    print!("Context: {context}");
    let context_tokens = model.tokenize_bytes(&context, false, true).unwrap();
    println!("Tokens: ");
    for token in &context_tokens {
        print!("{}({}) ", model.token_to_piece(*token), token.0);
    }
    let (bos, eos) = (model.bos(), model.eos());
    println!("bos: {}({})", model.token_to_piece(bos), bos.0);
    println!("eos: {}({})", model.token_to_piece(eos), eos.0);
    session
        .advance_context_with_tokens_async(context_tokens).await
        .map_err(|_| Error::Custom("Failed to advance context".into()))?;

    let completion = session
        .start_completing_with(StandardSampler::default(), 1024);
    // let discard_tokens = [model.bos(), model.eos()];
    // let filter_tokens = StreamExt::filter(completion, move |_token| !discard_tokens.contains(_token));
    let completion_stream = StreamExt::map(completion,  |token| Ok(format!("{}({}) ", model.token_to_piece(token), token.0)));
    // let completion_strings = TokensToStrings::new(filter_tokens, model.clone());
    // let completion_stream = StreamExt::map(completion_strings, Ok);

    Ok(Box::pin(completion_stream))
}