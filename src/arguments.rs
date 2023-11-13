use clap::{Parser, ValueEnum, Args, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<CliCommands>,
    #[command(flatten)]
    ask: Option<AskArgs>,
}
impl From<Cli> for CliCommands {
    fn from(args: Cli) -> Self {
        args.command.unwrap_or(CliCommands::Ask(args.ask.unwrap()))
    }
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    Ask(AskArgs),
    Config {
        /// Configuration file
        #[arg(long, default_value_t = format!("{1}{0}config.yml", std::path::MAIN_SEPARATOR, crate::filesystem::config_dir()))]
        config_path: String,
    },
    Creds {
        /// Credentials file
        #[arg(long, default_value_t = format!("{1}{0}creds.yml", std::path::MAIN_SEPARATOR, crate::filesystem::cache_dir()))]
        creds_path: String,
    }
}

/// Program to communicate with large language models and AI API 
#[derive(Args, Debug)]
pub struct AskArgs {
    /// Configuration file
    #[arg(long, default_value_t = format!("{1}{0}config.yml", std::path::MAIN_SEPARATOR, crate::filesystem::config_dir()))]
    pub config_path: String,
    /// Credentials file
    #[arg(long, default_value_t = format!("{1}{0}creds.yml", std::path::MAIN_SEPARATOR, crate::filesystem::cache_dir()))]
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
#[derive(Default, Debug, Clone)]
pub struct ProcessedArgs {
    pub config_path: String,
    pub creds_path: String,
    pub engine: String,
    pub formatter: FormatterChoice,
    pub run: RunChoice,
    pub input: String,
}

impl From<AskArgs> for ProcessedArgs {
    fn from(args: AskArgs) -> Self {
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