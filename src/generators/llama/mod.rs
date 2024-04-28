pub mod config;
pub mod template;

use tokio_stream::StreamExt;

use llama_cpp::{
    standard_sampler::StandardSampler, LlamaModel, LlamaParams, SessionParams, TokensToStrings
};
use once_cell::sync::OnceCell;
use log::debug;
use crate::{
    args, config::{format_content, Config as AIOConfig}
};
use super::{Error, ResultRun};

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

    let prompt = config.local.prompts.first().unwrap();
    let messages = prompt.content.iter()
        .cloned()
        .map(|mut m| {
            m.content = format_content(&m.content, &args).to_string(); 
            m
        })
        .collect::<Vec<_>>();
    let context_tokens = model_config.template.messages_to_tokens(model, &messages).map_err(|_| Error::Custom("Failed to convert prompt messages to tokens".into()))?;
    if log::log_enabled!(log::Level::Debug) {
        debug!("Tokens: ");
        for token in &context_tokens {
            print!("{}({})", String::from_utf8_lossy(model.detokenize(*token)), token.0);
        }
        println!();
        let (bos, eos) = (model.bos(), model.eos());
        debug!("Special tokens:");
        debug!("bos: {}({})", String::from_utf8_lossy(model.detokenize(bos)), bos.0);
        debug!("eos: {}({})", String::from_utf8_lossy(model.detokenize(eos)), eos.0);
    }
    session
        .advance_context_with_tokens_async(context_tokens).await
        .map_err(|_| Error::Custom("Failed to advance context".into()))?;

    let completion = session
        .start_completing_with(StandardSampler::default(), 1024);
    // let completion_stream = StreamExt::map(completion,  |token| Ok(format!("{}({})", model.token_to_piece(token), token.0)));
    let discard_tokens = model_config.template.stop_tokens(model).map_err(|_| Error::Custom("Failed to convert prompt messages to tokens".into()))?;
    let completion_stream = 
        StreamExt::map(
            TokensToStrings::new(
                StreamExt::take_while(completion, move |token| !discard_tokens.contains(token)), model.clone()
            ), 
            Ok
        );
    
    Ok(Box::pin(completion_stream))
}