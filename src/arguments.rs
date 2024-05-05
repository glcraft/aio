use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};

/// Program to communicate with large language models and AI API 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Configuration file
    #[arg(long, global = true, default_value_t = format!("{1}{0}config.yml", std::path::MAIN_SEPARATOR, crate::filesystem::config_dir()))]
    pub config_path: String,
    /// Credentials file
    #[arg(long, global = true, default_value_t = format!("{1}{0}creds.yml", std::path::MAIN_SEPARATOR, crate::filesystem::cache_dir()))]
    pub creds_path: String,
    /// Verbose mode
    /// 
    /// Count: 
    /// 0: errors
    /// 1: warnings
    /// 2: info
    /// 3: debug
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Engine name
    /// 
    /// The name can be followed by custom prompt name from the configuration file
    /// (ex: openai:command)
    #[command(subcommand)]
    pub engine: Subcommands,
    /// Formatter
    /// 
    /// Possible values: markdown, raw
    #[arg(long, short, global = true, value_enum, default_value_t = Default::default())]
    pub formatter: FormatterChoice,
    /// Run code block if the language is supported
    #[arg(long, short, global = true, value_enum, default_value_t = Default::default())]
    pub run: RunChoice,
    /// Force to run code 
    /// User text prompt
    #[arg(global = true, default_value_t = Default::default())]
    pub input: String,
}
#[derive(Subcommand, Debug, Clone)]
pub enum Subcommands {
    Api(ApiArgs),
    FromFile(FromFileArgs),
    Local(LocalArgs),
}

#[derive(ClapArgs, Debug, Clone)]
pub struct ApiArgs {
    #[arg(long, short)]
    pub model: String,
    #[arg(long, short)]
    pub prompt: Option<String>,
}
#[derive(ClapArgs, Debug, Clone)]
pub struct FromFileArgs;
#[derive(ClapArgs, Debug, Clone)]
pub struct LocalArgs {
    #[arg(long, short)]
    pub model: String,
    #[arg(long, short)]
    pub prompt: Option<String>,
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