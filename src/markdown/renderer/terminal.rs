use super::token;
use super::Renderer;

pub struct TerminalRenderer;

impl Renderer for TerminalRenderer {
    type BackendErrorType = ();
    fn push_token(&mut self, _style: token::Token) -> super::Result<(), Self::BackendErrorType> {
        todo!()
        Ok(())
    }
}