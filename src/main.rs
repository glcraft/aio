pub mod openai;
pub mod arguments;
mod printer;
mod config;
use arguments as args;
use clap::Parser;
// mod http2;
use std::{
    io::{
        Write
    }, str::FromStr
};

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    // let doc_markdown = std::fs::read_to_string("test.md").expect("Failed to read test.md");
    // std::fs::write("test_md_tokenized.txt", format!("{:#?}", markdown::tokenize(&doc_markdown))).expect("Failed to write to file");
    crossterm::queue!(std::io::stdout(), crossterm::style::Print("Hello")).map_err(|_| "Failed to write to stdout")?;
    // crossterm::queue!(std::io::stdout(), crossterm::cursor::MoveLeft(7)).map_err(|_| "Failed to write to stdout")?;
    crossterm::queue!(std::io::stdout(), crossterm::style::Print("World")).map_err(|_| "Failed to write to stdout")?;
    return Ok(());
    let mut prt = printer::MarkdownPrinter::new();
    let doc_markdown = std::fs::read_to_string("test.md").expect("Failed to read test.md");
    doc_markdown
        .split_inclusive(|c| !char::is_alphanumeric(c))
        .for_each(|s| {
            prt.push(s);
            std::thread::sleep(std::time::Duration::from_millis(100));
        });
    prt.flush();
    return Ok(());
    let args = args::Args::parse();
    let config = config::Config::load().expect("Failed to load config");
    if args.prompt == "?" {
        println!("Available prompts:");
        for prompt in config.prompts {
            println!("  - {}", prompt.name);
        }
        return Ok(());
    }
    let prompt = match config.prompts.into_iter()
        .find(|prompt| prompt.name == args.prompt) {
            Some(prompt) => prompt,
            None => {
                return Err("Prompt not found");
            }
        }.format_messages(&args);

    let openai_api_key = config.api_key
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .expect("OPENAI_API_KEY not set");
    // Send a request
    let chat_request = openai::ChatRequest::new("gpt-3.5-turbo".to_string())
        .add_messages(prompt.messages)
        .set_parameters(prompt.parameters.into())
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