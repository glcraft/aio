mod parser;
mod renderer;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StyleType {
    Bold,
    Italic,
    BoldItalic,
    Code,
    CodeBlock,
}

impl<S: AsRef<str>> From<S> for StyleType {
    fn from(s: S) -> Self {
        match s.as_ref() {
            "*" | "_" => StyleType::Italic,
            "**" | "__" => StyleType::Bold,
            "***" | "___" => StyleType::BoldItalic,
            "`" => StyleType::Code,
            "```" => StyleType::CodeBlock,
            _ => panic!("modifier not recognized")
        }
    }
}

impl From<StyleType> for &'static str {
    fn from(s: StyleType) -> Self {
        match s {
            StyleType::Italic => "*",
            StyleType::Bold => "**",
            StyleType::BoldItalic => "***",
            StyleType::Code => "`",
            StyleType::CodeBlock => "```",
        }
    }
}

pub use parser::Parser;
pub use renderer::{Renderer, TerminalRenderer};