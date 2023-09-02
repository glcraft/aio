mod markdown;
mod raw;

use anyhow::Result;

use raw::RawFormater;
pub type MarkdownFormatter = markdown::Parser<markdown::TerminalRenderer>;

pub trait Formatter {
    fn push(&mut self, text: &str) -> Result<()>;
    fn end_of_document(&mut self) -> Result<()> {
        Ok(())
    }
}

pub fn new_markdown_formatter() -> MarkdownFormatter {
    markdown::Parser::new(markdown::TerminalRenderer::new())
}
pub fn new_raw_formatter() -> RawFormater {
    RawFormater
}