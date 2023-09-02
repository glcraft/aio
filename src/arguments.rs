use std::fmt::Display;

use clap::{Parser, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FormatterChoice {
    Markdown,
    Raw,
}

impl Default for FormatterChoice {
    fn default() -> Self {
        use std::io::IsTerminal;
        if std::io::stdout().is_terminal() {
            FormatterChoice::Markdown
        } else {
            FormatterChoice::Raw
        }
    }
}

impl Display for FormatterChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatterChoice::Markdown => write!(f, "markdown"),
            FormatterChoice::Raw => write!(f, "raw"),
        }
    }
}

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
    /// Formatter
    /// 
    /// Possible values: markdown, raw
    #[arg(long, short, default_value_t = Default::default())]
    pub formatter: FormatterChoice,
    /// User text prompt
    pub input: String,
}