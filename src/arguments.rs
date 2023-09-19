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
    #[arg(long, short, value_enum, default_value_t = Default::default())]
    pub formatter: FormatterChoice,
    /// Run code block if the language is supported
    #[arg(long, short, value_enum, default_value_t = Default::default())]
    pub run: RunChoice,
    /// Force to run code 
    /// User text prompt
    pub input: Option<String>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum FormatterChoice {
    /// Markdown display
    #[default]
    Markdown,
    /// Raw display
    Raw,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum RunChoice {
    /// Doesn't run anything
    #[default]
    No,
    /// Ask to run code
    Ask,
    /// Run code without asking
    Force
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