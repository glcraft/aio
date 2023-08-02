use crossterm::{
    queue,
    ErrorKind
};
use std::io::Write;
use super::token;
use super::Renderer;

pub struct TerminalRenderer;

impl TerminalRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for TerminalRenderer {
    type Error = ErrorKind;
    fn push_token(&mut self, style: token::Token) -> Result<(), Self::Error> {
        match style {
            token::Token::Text(s) => queue!(std::io::stdout(), crossterm::style::Print(s)),
            token::Token::Newline => queue!(std::io::stdout(), crossterm::style::Print("\n")),
            token::Token::EndDocument => queue!(std::io::stdout(), crossterm::style::Print("\n")),
            _ => todo!(),
        }
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        std::io::stdout().flush()
    }
}