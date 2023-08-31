use clap::Parser;

/// Program to communicate with large language models and AI API 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Configuration file
    #[arg(long, default_value_t = String::from("~/.config/aio/config.yaml"))]
    pub config_path: String,
    /// Credentials file
    #[arg(long, default_value_t = String::from("~/.config/aio/creds.yaml"))]
    pub creds_path: String,
    /// Engine name
    /// 
    /// The name can be followed by custom prompt name from the configuration file
    /// (ex: openai:command)
    #[arg(long, short)]
    pub engine: String,
    /// User text prompt
    pub input: String,
}