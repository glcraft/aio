mod markdown;

pub type MarkdownFormatter = markdown::Parser<markdown::TerminalRenderer>;

pub trait Formatter {
    type Error;
    fn push(&mut self, text: &str) -> Result<(), Self::Error>;
    fn end_of_document(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn new_markdown_formatter() -> MarkdownFormatter {
    markdown::Parser::new(markdown::TerminalRenderer::new())
}