use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Configuration file
    /// Default: ~/.aio/config.yaml
    #[arg(long, default_value_t = String::from("~/.aio/config.yaml"))]
    pub config_path: String,
    /// Credentials file
    /// Default: ~/.aio/creds.yaml
    #[arg(long, default_value_t = String::from("~/.aio/creds.yaml"))]
    pub creds_path: String,
    /// Engine name
    /// The name can be followed by custom prompt name from the configuration file
    /// Ex: openai:command
    #[arg(long, short)]
    pub engine: String,
    /// Input
    pub input: String,
}