mod markdown;

pub type MarkdownFormatter = markdown::Parser<markdown::TerminalRenderer>;

pub fn new_markdown_formatter() -> MarkdownFormatter {
    markdown::Parser::new(markdown::TerminalRenderer::new())
}