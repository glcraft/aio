use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Prompt name
    pub prompt: String,
    /// Input
    pub input: String,
}