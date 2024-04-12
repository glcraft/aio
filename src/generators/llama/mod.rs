pub mod config;
use tokio::sync::Mutex;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use once_cell::sync::{Lazy, OnceCell};
use crate::{
    config::Config as AIOConfig,
    args
};
use super::{ResultRun, Error};

struct SendLLama(LLama);

unsafe impl Send for SendLLama {}


static LOCAL_LLAMA: OnceCell<Mutex<SendLLama>> = OnceCell::new();

fn init_model(config: &crate::config::Config) -> Result<(), Error> {
    let model_options = ModelOptions {
        n_gpu_layers: 20000,
        ..Default::default()
    };
    let Ok(llama) = LLama::new(
        config.llama.model_path.clone(),
        &model_options,
    ) else {
        return Err(Error::Custom("Failed to load LLaMA model".into()))
    };
    let send_llama = SendLLama(llama);
    LOCAL_LLAMA.set(Mutex::new(send_llama)).map_err(|_| Error::Custom("Failed to set LLaMA model".into()))
}

pub async fn run(
    config: AIOConfig, 
    args: args::ProcessedArgs
) -> ResultRun {
    if LOCAL_LLAMA.get().is_none() {
        init_model(&config)?;
    }
    let llama = LOCAL_LLAMA.get().unwrap().lock().await;
    let llama = &llama.0;

    let (send, recv) = tokio::sync::mpsc::channel(10);

    let predict_options = PredictOptions {
        token_callback: Some(Box::new(move |token| {
            use tokio::runtime::Handle;

            // let send = send.clone();
            // tokio::spawn(async move {
            //     if let Err(e) = send.send(token).await {
            //         eprintln!("Failed to send token: {}", e);
            //     } else {
            //         println!("token sent");
            //     }
            // });
            print!("{}", token);

            true
        })),
        tokens: 0,
        threads: 14,
        top_k: 90,
        top_p: 0.8,
        debug_mode: false,
        ..Default::default()
    };
    llama
        .predict(
            args.input,
             predict_options,
        )
        .unwrap();
    
    let stream = ReceiverStream::new(recv).map(Ok);

    // send.send("test 1.2.3|".to_string()).await.expect("Failed to send");
    // send.send("test 4.5.6|".to_string()).await.expect("Failed to send");
    // send.send("test 7.8.9|".to_string()).await.expect("Failed to send");
    Ok(Box::pin(stream))
}