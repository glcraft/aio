use std::fmt::Display;

use clap::{Parser, ValueEnum};

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
    /// Run code block if the language is supported
    #[arg(long, short, default_value_t = Default::default())]
    pub run: RunChoice,
    /// Force to run code 
    /// User text prompt
    pub input: Option<String>,
}

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

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RunChoice {
    /// Doesn't run anything
    #[default]
    No,
    /// Ask to run code
    Ask,
    /// Run code without asking
    Force
}

impl Display for RunChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunChoice::No => write!(f, "no"),
            RunChoice::Ask => write!(f, "ask"),
            RunChoice::Force => write!(f, "force"),
        }
    }
}
pub struct ProcessedArgs {
    pub config_path: String,
    pub creds_path: String,
    pub engine: String,
    pub formatter: FormatterChoice,
    pub run: RunChoice,
    pub input: String,
}

impl From<Args> for ProcessedArgs {
    fn from(args: Args) -> Self {
        Self {
            config_path: args.config_path,
            creds_path: args.creds_path,
            engine: args.engine,
            formatter: args.formatter,
            run: args.run,
            input: args.input.unwrap_or_default(),
        }
    }
}