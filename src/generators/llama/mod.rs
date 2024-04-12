pub mod config;
use tokio::sync::Mutex;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use llama_cpp::{
    LlamaModel, LlamaParams, SessionParams,TokensToStrings,
    standard_sampler::StandardSampler
};
use once_cell::sync::{Lazy, OnceCell};
use crate::{
    config::Config as AIOConfig,
    args
};
use super::{ResultRun, Error};

static LOCAL_LLAMA: OnceCell<LlamaModel> = OnceCell::new();

fn init_model(config: &crate::config::Config) -> Result<(), Error> {
    let model_options = LlamaParams {
        n_gpu_layers: 20000,
        ..Default::default()
    };
    let Ok(llama) = LlamaModel::load_from_file(
        &config.llama.model_path,
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
    if LOCAL_LLAMA.get().is_none() {
        init_model(&config)?;
    }
    let model = LOCAL_LLAMA.get().unwrap();

    // let (send, recv) = tokio::sync::mpsc::channel(10);
    let prompt = args.input;
    let session_params = SessionParams::default();
    let mut session = model.create_session(session_params).map_err(|_| Error::Custom("Failed to create session".into()))?;

    session
        .advance_context_async(prompt).await
        .map_err(|_| Error::Custom("Failed to advance context".into()))?;

    let completion = session
        .start_completing_with(StandardSampler::default(), 1024);
    let discard_tokens = [model.bos(), model.eos()];
    let filter_tokens = StreamExt::filter(completion, move |_token| !discard_tokens.contains(_token));
    let completion_strings = TokensToStrings::new(filter_tokens, model.clone());
    let completion_stream = StreamExt::map(completion_strings,  Ok);

    Ok(Box::pin(completion_stream))
}