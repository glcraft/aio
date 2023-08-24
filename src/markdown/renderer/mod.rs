mod terminal;

use super::token;
pub use terminal::TerminalRenderer;

pub trait Renderer {
    type Error: std::fmt::Debug;
    fn push_token(&mut self, style: token::Token) -> Result<(), Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}
