use super::token;
use super::Renderer;

pub struct TerminalRenderer;

impl TerminalRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for TerminalRenderer {
    type Error = ();
    fn push_token(&mut self, _style: token::Token) -> Result<(), Self::Error> {
        todo!();
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        todo!();
    }
}