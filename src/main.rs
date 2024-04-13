pub mod arguments;
mod utils;
mod config;
mod credentials;
mod filesystem;
mod formatters;
mod generators;
mod runner;
mod serde_io;

mod openai {}

use arguments as args;
use clap::Parser;
use formatters::Formatter;
use serde_io::DeserializeExt;
use tokio_stream::StreamExt;

macro_rules! raise_str {
    ($expr:expr) => {
        raise_str!($expr, "{}")
    };
    ($expr:expr, $text:literal) => {{
        $expr.map_err(|e| format!($text, e))?
    }};
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = {
        let mut args = args::Args::parse();
        if args.input.is_none() {
            use std::io::Read;
            let mut str_input = std::string::String::new();
            let mut stdin = std::io::stdin();
            stdin
                .read_to_string(&mut str_input)
                .map_err(|e| format!("Failed to read input from stdin: {}", e))?;

            args.input = Some(str_input.trim().to_string());
        }
        args::ProcessedArgs::from(args)
    };
    let config =
        config::Config::from_yaml_file(filesystem::resolve_path(&args.config_path).as_ref())
            .map_err(|e| {
                format!(
                    "An error occured while loading or creating configuration file: {}",
                    e
                )
            })?;
    let creds = raise_str!(
        credentials::Credentials::from_yaml_file(
            filesystem::resolve_path(&args.creds_path).as_ref()
        ),
        "Failed to parse credentials file: {}"
    );

    let mut formatter: Box<dyn Formatter> = match args.formatter {
        args::FormatterChoice::Markdown => Box::new(formatters::new_markdown_formatter()),
        args::FormatterChoice::Raw => Box::new(formatters::new_raw_formatter()),
    };
    let mut runner = runner::Runner::new(args.run);

    let (engine, _prompt) = args
        .engine
        .find(':')
        .map(|i| (&args.engine[..i], Some(&args.engine[i + 1..])))
        .unwrap_or((args.engine.as_str(), None));

    let mut stream = match engine {
        "openai" => generators::openai::run(creds.openai, config, args).await,
        "local" => generators::llama::run(config, args).await,
        "from-file" => generators::from_file::run(config, args).await,
        _ => panic!("Unknown engine: {}", engine),
    }
    .map_err(|e| format!("Failed to request OpenAI API: {}", e))?;

    loop {
        match stream.next().await {
            Some(Ok(token)) => {
                raise_str!(formatter.push(&token), "Failed to parse markdown: {}");
                raise_str!(
                    runner.push(&token),
                    "Failed push text in the runner system: {}"
                );
            }
            Some(Err(e)) => Err(e.to_string())?,
            None => break,
        }
    }
    raise_str!(formatter.end_of_document(), "Failed to end markdown: {}");
    raise_str!(runner.end_of_document(), "Failed to run code: {}");
    Ok(())
}

