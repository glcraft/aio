pub mod arguments;
mod generators;
mod markdown;
mod config;
mod credentials;
mod serde_io;

use arguments as args;
use clap::Parser;
use serde_io::DeserializeExt;
use smartstring::alias::String;
use tokio_stream::StreamExt;

macro_rules! raise_str {
    ($expr:expr) => {
        raise_str!($expr, "{}")
    };
    ($expr:expr, $text:literal) => {
        { $expr.map_err(|e| format!($text, e))? }
    };
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let term_renderer = markdown::TerminalRenderer::new();
    let mut md_parser = markdown::Parser::new(term_renderer);

    let args = args::Args::parse();
    let config = raise_str!(
        config::Config::from_yaml_file(&args.config_path),
        "Failed to parse config file: {}"
    );
    let creds = raise_str!(
        credentials::Credentials::from_yaml_file(&args.creds_path),
        "Failed to parse credentials file: {}"
    );

    let engine = args.engine
        .find(':')
        .map(|i| &args.engine[..i])
        .unwrap_or(args.engine.as_str());
    let mut stream = match engine {
        "openai" => generators::openai::run(creds.openai, config, args).await,
        _ => panic!("Unknown engine: {}", engine),
    }.map_err(|e| format!("Failed to request OpenAI API: {}", e))?;

    loop {
        match stream.next().await {
            Some(Ok(token)) => raise_str!(md_parser.push(&token), "Failed to parse markdown: {}"),
            Some(Err(e)) => Err(e.to_string())?,
            None => break,
        }
    }
    raise_str!(md_parser.end_of_document(), "Failed to end markdown: {}");
    Ok(())
}