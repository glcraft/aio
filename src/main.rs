mod openai;
// mod http2;
use std::{
    io::{
        Write
    }, str::FromStr
};

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    // Send a request
    let chat_request = openai::ChatRequest::new("gpt-3.5-turbo".to_string())
        .add_message(openai::Role::System, "You are ChatGPT, a powerful conversational chatbot. Answer to my questions in informative way.".to_string())
        .add_message(openai::Role::User, "Hi, how are you ?".to_string())
        .into_stream();

    let client = reqwest::Client::new();
    let mut stream = client.post("https://api.openai.com/v1/chat/completions")
        .header("User-Agent", "openai-rs/1.0")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&chat_request)
        .send()
        .await
        .expect("Failed to send request")
        .bytes_stream();
    
    while let Some(item) = stream.next().await
        .map(Result::ok)
        .flatten() {
            // println!("stream received");
            String::from_utf8_lossy(item.as_ref())
                .split("\n\n")
                .filter(|item| !item.is_empty())
                .map(openai::ChatResponse::from_str)
                .filter_map(|item| {
                    match item {
                        Ok(item) => Some(item),
                        Err(e) => {
                            if cfg!(debug_assertions) {
                                println!("Error: {:?}", e); 
                            }
                            None
                        },
                    }
                })
                .for_each(|item| {
                    print!("{}", item);
                    if let Err(e) = std::io::stdout().flush() {
                        if cfg!(debug_assertions) {
                            println!("Error: {:?}", e); 
                        }
                    };
                })
    }
    Ok(())
}