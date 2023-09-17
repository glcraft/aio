pub mod arguments;
mod runner;
mod generators;
mod formatters;
mod config;
mod credentials;
mod serde_io;
use std::borrow::Cow;

use clap::Parser;
use serde_io::DeserializeExt;
use tokio_stream::StreamExt;
use arguments as args;
use formatters::Formatter;

macro_rules! raise_str {
    ($expr:expr) => {
        raise_str!($expr, "{}")
    };
    ($expr:expr, $text:literal) => {
        { $expr.map_err(|e| format!($text, e))? }
    };
}

fn resolve_path(path: &str) -> Cow<str> {
    if path.starts_with("~/") {
        #[cfg(unix)]
        let home = std::env::var("HOME").expect("Failed to resolve home path");
        #[cfg(windows)]
        let home = std::env::var("USERPROFILE").expect("Failed to resolve user profile path");
        Cow::Owned(format!("{}{}{}", home, std::path::MAIN_SEPARATOR, &path[2..]))
    } else {
        Cow::Borrowed(path)
    }
}

fn get_config<P: AsRef<std::path::Path>>(path: P) -> Result<config::Config, String> {
    let path = path.as_ref();
    if path.exists() {
        return config::Config::from_yaml_file(path).map_err(|e| e.to_string())
    }
    use std::io::Write;
    let default_config = config::Config::default();
    let yaml = serde_yaml::to_string(&default_config).map_err(|e| e.to_string())?;
    std::fs::File::create(path).unwrap().write_all(yaml.as_bytes()).map_err(|e| e.to_string())?;
    Ok(default_config)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = {
        let mut args = args::Args::parse();
        if let None = args.input {
            use std::io::Read;
            let mut str_input = std::string::String::new();
            std::io::stdin().lock().read_to_string(&mut str_input).map_err(|e| format!("Failed to read input from stdin: {}", e))?;
            args.input = Some(str_input);
        }
        args::ProcessedArgs::from(args)
    };
    let config = get_config(resolve_path(&args.config_path).as_ref())?;
    let creds = raise_str!(
        credentials::Credentials::from_yaml_file(resolve_path(&args.creds_path).as_ref()),
        "Failed to parse credentials file: {}"
    );

    let mut formatter: Box<dyn Formatter> = match args.formatter {
        args::FormatterChoice::Markdown => Box::new(formatters::new_markdown_formatter()),
        args::FormatterChoice::Raw => Box::new(formatters::new_raw_formatter()),
    };
    let mut runner = execution::Executor::new();

    let (engine, prompt) = args.engine
        .find(':')
        .map(|i| (&args.engine[..i], Some(&args.engine[i+1..])))
        .unwrap_or((args.engine.as_str(), None));

    let mut stream = match engine {
        "openai" => generators::openai::run(creds.openai, config, args).await,
        "debug" => generators::debug::run(config, args).await,
        _ => panic!("Unknown engine: {}", engine),
    }.map_err(|e| format!("Failed to request OpenAI API: {}", e))?;

    loop {
        match stream.next().await {
            Some(Ok(token)) => {
                raise_str!(formatter.push(&token), "Failed to parse markdown: {}");
                raise_str!(runner.push(&token), "Failed push text for execution: {}");
            },
            Some(Err(e)) => Err(e.to_string())?,
            None => break,
        }
    }
    raise_str!(formatter.end_of_document(), "Failed to end markdown: {}");
    Ok(())
}