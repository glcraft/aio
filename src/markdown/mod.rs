mod parser;
mod renderer;
use smartstring::alias::String;

#[derive(Debug, PartialEq, Clone)]
pub enum StyleKind {
    Bold,
    Italic,
    BoldItalic,
    InlineCode,
    CodeBlock{
        language: Option<String>,
    },
    Quote,
    Heading(u8),
    ListItem{
        level: u8,
        marker: String,
    },
}

pub use parser::Parser;
pub use renderer::{Renderer, TerminalRenderer};