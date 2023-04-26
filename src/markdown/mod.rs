mod parser;
mod renderer;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InlineStyleType {
    Bold,
    Italic,
    BoldItalic,
    Code,
    CodeBlock,
}


impl<S: AsRef<str>> From<S> for InlineStyleType {
    fn from(s: S) -> Self {
        match s.as_ref() {
            "*" | "_" => InlineStyleType::Italic,
            "**" | "__" => InlineStyleType::Bold,
            "***" | "___" => InlineStyleType::BoldItalic,
            "`" => InlineStyleType::Code,
            _ => panic!("modifier not recognized")
        }
    }
}

impl From<InlineStyleType> for &'static str {
    fn from(s: InlineStyleType) -> Self {
        match s {
            InlineStyleType::Italic => "*",
            InlineStyleType::Bold => "**",
            InlineStyleType::BoldItalic => "***",
            InlineStyleType::Code => "`",
        }
    }
}

pub use parser::Parser;
pub use renderer::{Renderer, TerminalRenderer};